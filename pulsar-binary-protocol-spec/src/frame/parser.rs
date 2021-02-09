use std::convert::{TryFrom, TryInto};

use crc32c::crc32c_append;
use protobuf::{Message as _, ProtobufEnum as _, ProtobufError};
use thiserror::Error;

use crate::{
    command::{
        CommandWithParsed, PayloadCommandPayloadWithParsed, PayloadCommandWithParsed, SimpleCommand,
    },
    protos::protobuf::pulsar_api::{BaseCommand, MessageMetadata, SingleMessageMetadata},
    types::CompressionType,
};

use super::{MAGIC_NUMBER, MAX_FRAME_SIZE_DEFAULT};

#[derive(Default, Debug, Clone)]
pub struct FrameParserConfig {
    max_frame_size: Option<u32>,
    compression_buf_capacity: Option<u32>,
    // TODO, check ProtocolVersion
}
impl FrameParserConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_max_frame_size(&mut self, value: u32) -> &mut Self {
        self.max_frame_size = Some(value);
        self
    }

    fn get_max_frame_size(&self) -> u32 {
        self.max_frame_size.unwrap_or(MAX_FRAME_SIZE_DEFAULT)
    }

    pub fn set_compression_buf_capacity(&mut self, value: u32) -> &mut Self {
        self.compression_buf_capacity = Some(value);
        self
    }

    fn get_compression_buf_capacity(&self) -> u32 {
        self.compression_buf_capacity.unwrap_or(2 * 1024 * 1024)
    }
}

#[derive(Debug)]
pub enum FrameParseOutput {
    Completed(usize, CommandWithParsed),
    Partial(usize),
}

#[derive(Error, Debug)]
pub enum FrameParseError {
    #[error("PayloadTooLarge current:{current} max:{max}")]
    PayloadTooLarge { current: u32, max: u32 },

    #[error("TotalSizeInvalid")]
    TotalSizeInvalid,

    #[error("CommandSizeInvalid")]
    CommandSizeInvalid,

    #[error("DeserializeMessageError {0:?}")]
    DeserializeMessageError(ProtobufError),

    #[error("MagicNumberInvalid")]
    MagicNumberInvalid,

    #[error("ChecksumInvalid")]
    ChecksumInvalid,

    #[error("MetadataSizeInvalid")]
    MetadataSizeInvalid,

    #[error("DeserializeMetadataError {0:?}")]
    DeserializeMetadataError(ProtobufError),

    #[error("MetadataInvalid {0}")]
    MetadataInvalid(String),
}

#[derive(Error, Debug)]
pub enum FrameParseSinglePayloadError {
    #[error("CompressionUnsupported type:{type_code}")]
    CompressionUnsupported { type_code: i32 },

    #[cfg(feature = "with-compression-lz4")]
    #[error("CompressionLZ4DecompressError {0}")]
    CompressionLZ4DecompressError(std::io::Error),

    #[cfg(feature = "with-compression-zlib")]
    #[error("CompressionZlibDecompressError {0}")]
    CompressionZlibDecompressError(std::io::Error),

    #[error("UncompressedSizeCorruption")]
    UncompressedSizeCorruption,
}

#[derive(Error, Debug)]
pub enum FrameParseBatchPayloadError {
    #[error("CompressionUnsupported type:{type_code}")]
    CompressionUnsupported { type_code: i32 },

    #[cfg(feature = "with-compression-lz4")]
    #[error("CompressionLZ4DecompressError {0}")]
    CompressionLZ4DecompressError(std::io::Error),

    #[cfg(feature = "with-compression-zlib")]
    #[error("CompressionZlibDecompressError {0}")]
    CompressionZlibDecompressError(std::io::Error),

    #[error("UncompressedSizeCorruption")]
    UncompressedSizeCorruption,

    #[error("GetSingleMessageMetadataSizeFailed")]
    GetSingleMessageMetadataSizeFailed,

    #[error("GetSingleMessageMetadataFailed")]
    GetSingleMessageMetadataFailed,

    #[error("DeserializeSingleMessageMetadataError {0:?}")]
    DeserializeSingleMessageMetadataError(ProtobufError),

    #[error("GetSingleMessagePayloadFailed")]
    GetSingleMessagePayloadFailed,
}

#[derive(Default, Debug, Clone)]
pub struct FrameParser {
    total_size: u32,
    command_size: u32,
    message: BaseCommand,
    magic_number: u16,
    checksum: u32,
    pending_checksum: u32,
    metadata_size: u32,
    metadata: MessageMetadata,
    //
    config: FrameParserConfig,
    compression_buf: Vec<u8>,
    //
    state: State,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum State {
    Idle,
    TotalSizeParsed,
    CommandSizeParsed,
    MessageParsed,
    MagicNumberParsed,
    ChecksumParsed,
    MetadataSizeParsed,
    MetadataParsed,
}
impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

impl FrameParser {
    pub fn new() -> Self {
        Self::with_config(Default::default())
    }

    pub fn with_config(config: FrameParserConfig) -> Self {
        let compression_buf_capacity = config.get_compression_buf_capacity();
        Self {
            config,
            compression_buf: Vec::with_capacity(compression_buf_capacity as usize),
            ..Default::default()
        }
    }

    pub fn get_mut_config(&mut self) -> &mut FrameParserConfig {
        &mut self.config
    }

    pub fn parse(&mut self, slice: &[u8]) -> Result<FrameParseOutput, FrameParseError> {
        let mut n_parsed = 0;

        if self.state < State::TotalSizeParsed {
            if slice.len() - n_parsed < 4 {
                return Ok(FrameParseOutput::Partial(n_parsed));
            }

            let total_size =
                u32::from_be_bytes((&slice[n_parsed..n_parsed + 4]).try_into().expect(""));
            n_parsed += 4;

            if total_size < 4 {
                return Err(FrameParseError::TotalSizeInvalid);
            }

            if total_size > self.config.get_max_frame_size() - 4 {
                return Err(FrameParseError::PayloadTooLarge {
                    current: total_size,
                    max: self.config.get_max_frame_size() - 4,
                });
            }

            self.total_size = total_size;
            self.state = State::TotalSizeParsed;
        }

        if self.state < State::CommandSizeParsed {
            if slice.len() - n_parsed < 4 {
                return Ok(FrameParseOutput::Partial(n_parsed));
            }

            let command_size =
                u32::from_be_bytes((&slice[n_parsed..n_parsed + 4]).try_into().expect(""));
            n_parsed += 4;

            if command_size == 0 {
                return Err(FrameParseError::CommandSizeInvalid);
            }

            if command_size > self.total_size - 4 {
                return Err(FrameParseError::CommandSizeInvalid);
            }

            self.command_size = command_size;
            self.state = State::CommandSizeParsed;
        }

        if self.state < State::MessageParsed {
            if slice.len() - n_parsed < self.command_size as usize {
                return Ok(FrameParseOutput::Partial(n_parsed));
            }

            self.message = BaseCommand::parse_from_bytes(
                &slice[n_parsed..n_parsed + self.command_size as usize],
            )
            .map_err(FrameParseError::DeserializeMessageError)?;
            n_parsed += self.command_size as usize;

            self.state = State::MessageParsed;

            if self.total_size - 4 == self.command_size {
                self.state = State::Idle;

                let command = CommandWithParsed::Simple(SimpleCommand {
                    message: self.message.to_owned(),
                });
                return Ok(FrameParseOutput::Completed(n_parsed, command));
            }
        }

        if self.state < State::MagicNumberParsed {
            if slice.len() - n_parsed < 2 {
                return Ok(FrameParseOutput::Partial(n_parsed));
            }

            let magic_number =
                u16::from_be_bytes((&slice[n_parsed..n_parsed + 2]).try_into().expect(""));
            n_parsed += 2;

            if magic_number != MAGIC_NUMBER {
                return Err(FrameParseError::MagicNumberInvalid);
            }

            self.magic_number = magic_number;
            self.state = State::MagicNumberParsed;
        }

        if self.state < State::ChecksumParsed {
            if slice.len() - n_parsed < 4 {
                return Ok(FrameParseOutput::Partial(n_parsed));
            }

            let checksum =
                u32::from_be_bytes((&slice[n_parsed..n_parsed + 4]).try_into().expect(""));
            n_parsed += 4;

            if checksum == 0 {
                return Err(FrameParseError::ChecksumInvalid);
            }

            self.checksum = checksum;
            self.pending_checksum = 0;
            self.state = State::ChecksumParsed;
        }

        if self.state < State::MetadataSizeParsed {
            if slice.len() - n_parsed < 4 {
                return Ok(FrameParseOutput::Partial(n_parsed));
            }

            let metadata_size =
                u32::from_be_bytes((&slice[n_parsed..n_parsed + 4]).try_into().expect(""));
            self.pending_checksum =
                crc32c_append(self.pending_checksum, &slice[n_parsed..n_parsed + 4]);
            n_parsed += 4;

            if metadata_size == 0 {
                return Err(FrameParseError::MetadataSizeInvalid);
            }

            if metadata_size > self.total_size - 4 - self.command_size - 2 - 4 - 4 {
                return Err(FrameParseError::MetadataSizeInvalid);
            }

            self.metadata_size = metadata_size;
            self.state = State::MetadataSizeParsed;
        }

        if self.state < State::MetadataParsed {
            if slice.len() - n_parsed < self.metadata_size as usize {
                return Ok(FrameParseOutput::Partial(n_parsed));
            }

            self.metadata = MessageMetadata::parse_from_bytes(
                &slice[n_parsed..n_parsed + self.metadata_size as usize],
            )
            .map_err(FrameParseError::DeserializeMetadataError)?;
            self.pending_checksum = crc32c_append(
                self.pending_checksum,
                &slice[n_parsed..n_parsed + self.metadata_size as usize],
            );
            n_parsed += self.metadata_size as usize;

            if self.metadata.get_num_messages_in_batch() < 0 {
                return Err(FrameParseError::MetadataInvalid(
                    "num_messages_in_batch should gt 0".to_owned(),
                ));
            }

            self.state = State::MetadataParsed;
        }

        //
        let payload_size = self.total_size - 4 - self.command_size - 2 - 4 - 4 - self.metadata_size;

        if slice.len() - n_parsed < payload_size as usize {
            return Ok(FrameParseOutput::Partial(n_parsed));
        }
        let payload_slice = &slice[n_parsed..n_parsed + payload_size as usize];
        self.pending_checksum = crc32c_append(
            self.pending_checksum,
            &slice[n_parsed..n_parsed + payload_size as usize],
        );
        n_parsed += payload_size as usize;

        let is_checksum_match = Some(self.pending_checksum == self.checksum);

        if self.metadata.get_num_messages_in_batch() == 0 && payload_size != 0 {
            return Err(FrameParseError::MetadataInvalid(
                "num_messages_in_batch should eq 0".to_owned(),
            ));
        }

        if self.metadata.get_num_messages_in_batch() == 1 {
            let command = self.make_payload_command_for_single(payload_slice, is_checksum_match);
            self.state = State::Idle;
            return Ok(FrameParseOutput::Completed(n_parsed, command));
        }

        let command = self.make_payload_command_for_batch(
            payload_slice,
            self.metadata.get_num_messages_in_batch() as u32,
            is_checksum_match,
        );
        self.state = State::Idle;
        Ok(FrameParseOutput::Completed(n_parsed, command))
    }

    pub fn get_total_size(&self) -> Option<u32> {
        if self.state >= State::TotalSizeParsed {
            Some(self.total_size)
        } else {
            None
        }
    }

    fn make_payload_command_for_single(
        &mut self,
        payload_slice: &[u8],
        is_checksum_match: Option<bool>,
    ) -> CommandWithParsed {
        let compression_type = self.metadata.get_compression();

        let payload_slice = match CompressionType::try_from(compression_type) {
            Ok(compression_type) if compression_type == CompressionType::NONE => payload_slice,
            Ok(compression_type) => {
                self.compression_buf.clear();

                match compression_type {
                    CompressionType::NONE => {}
                    #[cfg(feature = "with-compression-lz4")]
                    CompressionType::LZ4 => {
                        match crate::compression::lz4::decompress(payload_slice, &mut self.compression_buf)
                        {
                            Ok(_) => {}
                            Err(err) => {
                                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                                    message: self.message.to_owned(),
                                    metadata: self.metadata.to_owned(),
                                    payload: PayloadCommandPayloadWithParsed::Single(Err(
                                        FrameParseSinglePayloadError::CompressionLZ4DecompressError(err)
                                            .into(),
                                    )),
                                    is_checksum_match,
                                }))
                            }
                        }
                    }
                    #[cfg(feature = "with-compression-zlib")]
                    CompressionType::ZLIB => {
                        match crate::compression::zlib::decompress(payload_slice, &mut self.compression_buf)
                        {
                            Ok(_) => {}
                            Err(err) => {
                                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                                    message: self.message.to_owned(),
                                    metadata: self.metadata.to_owned(),
                                    payload: PayloadCommandPayloadWithParsed::Single(Err(
                                        FrameParseSinglePayloadError::CompressionZlibDecompressError(err)
                                            .into(),
                                    )),
                                    is_checksum_match,
                                }))
                            }
                        }
                    }
                };

                if self.compression_buf.len() != self.metadata.get_uncompressed_size() as usize {
                    return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                        message: self.message.to_owned(),
                        metadata: self.metadata.to_owned(),
                        payload: PayloadCommandPayloadWithParsed::Single(Err(
                            FrameParseSinglePayloadError::UncompressedSizeCorruption.into(),
                        )),
                        is_checksum_match,
                    }));
                }

                &self.compression_buf[..]
            }
            Err(_) => {
                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                    message: self.message.to_owned(),
                    metadata: self.metadata.to_owned(),
                    payload: PayloadCommandPayloadWithParsed::Single(Err(
                        FrameParseSinglePayloadError::CompressionUnsupported {
                            type_code: compression_type.value(),
                        }
                        .into(),
                    )),
                    is_checksum_match,
                }))
            }
        };

        CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
            message: self.message.to_owned(),
            metadata: self.metadata.to_owned(),
            payload: PayloadCommandPayloadWithParsed::Single(Ok(payload_slice.to_owned())),
            is_checksum_match,
        }))
    }

    fn make_payload_command_for_batch(
        &mut self,
        payload_slice: &[u8],
        num_messages_in_batch: u32,
        is_checksum_match: Option<bool>,
    ) -> CommandWithParsed {
        let compression_type = self.metadata.get_compression();

        let payload_slice = match CompressionType::try_from(compression_type) {
            Ok(compression_type) if compression_type == CompressionType::NONE => payload_slice,
            Ok(compression_type) => {
                self.compression_buf.clear();

                match compression_type {
                    CompressionType::NONE => {}
                    #[cfg(feature = "with-compression-lz4")]
                    CompressionType::LZ4 => {
                        match crate::compression::lz4::decompress(
                            payload_slice,
                            &mut self.compression_buf,
                        ) {
                            Ok(_) => {}
                            Err(err) => return CommandWithParsed::Payload(Box::new(
                                PayloadCommandWithParsed {
                                    message: self.message.to_owned(),
                                    metadata: self.metadata.to_owned(),
                                    payload: PayloadCommandPayloadWithParsed::Batch(Err(
                                        FrameParseBatchPayloadError::CompressionLZ4DecompressError(
                                            err,
                                        )
                                        .into(),
                                    )),
                                    is_checksum_match,
                                },
                            )),
                        }
                    }
                    #[cfg(feature = "with-compression-zlib")]
                    CompressionType::ZLIB => {
                        match crate::compression::zlib::decompress(payload_slice, &mut self.compression_buf)
                        {
                            Ok(_) => {}
                            Err(err) => {
                                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                                    message: self.message.to_owned(),
                                    metadata: self.metadata.to_owned(),
                                    payload: PayloadCommandPayloadWithParsed::Batch(Err(
                                        FrameParseBatchPayloadError::CompressionZlibDecompressError(err)
                                            .into(),
                                    )),
                                    is_checksum_match,
                                }))
                            }
                        }
                    }
                };

                if self.compression_buf.len() != self.metadata.get_uncompressed_size() as usize {
                    return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                        message: self.message.to_owned(),
                        metadata: self.metadata.to_owned(),
                        payload: PayloadCommandPayloadWithParsed::Batch(Err(
                            FrameParseBatchPayloadError::UncompressedSizeCorruption.into(),
                        )),
                        is_checksum_match,
                    }));
                }

                &self.compression_buf[..]
            }
            Err(_) => {
                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                    message: self.message.to_owned(),
                    metadata: self.metadata.to_owned(),
                    payload: PayloadCommandPayloadWithParsed::Batch(Err(
                        FrameParseBatchPayloadError::CompressionUnsupported {
                            type_code: compression_type.value(),
                        }
                        .into(),
                    )),
                    is_checksum_match,
                }))
            }
        };

        //
        let mut n_parsed = 0;

        let mut msgs = vec![];
        for _ in 0..num_messages_in_batch {
            if payload_slice.len() - n_parsed < 4 {
                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                    message: self.message.to_owned(),
                    metadata: self.metadata.to_owned(),
                    payload: PayloadCommandPayloadWithParsed::Batch(Err(
                        FrameParseBatchPayloadError::GetSingleMessageMetadataSizeFailed.into(),
                    )),
                    is_checksum_match,
                }));
            }

            let single_message_metadata_size = u32::from_be_bytes(
                (&payload_slice[n_parsed..n_parsed + 4])
                    .try_into()
                    .expect(""),
            );
            n_parsed += 4;

            if payload_slice.len() - n_parsed < single_message_metadata_size as usize {
                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                    message: self.message.to_owned(),
                    metadata: self.metadata.to_owned(),
                    payload: PayloadCommandPayloadWithParsed::Batch(Err(
                        FrameParseBatchPayloadError::GetSingleMessageMetadataFailed.into(),
                    )),
                    is_checksum_match,
                }));
            }

            let single_message_metadata = match SingleMessageMetadata::parse_from_bytes(
                &payload_slice[n_parsed..n_parsed + single_message_metadata_size as usize],
            ) {
                Ok(single_message_metadata) => single_message_metadata,
                Err(err) => {
                    return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                        message: self.message.to_owned(),
                        metadata: self.metadata.to_owned(),
                        payload: PayloadCommandPayloadWithParsed::Batch(Err(
                            FrameParseBatchPayloadError::DeserializeSingleMessageMetadataError(err)
                                .into(),
                        )),
                        is_checksum_match,
                    }));
                }
            };
            n_parsed += single_message_metadata_size as usize;

            let single_message_payload_size = single_message_metadata.get_payload_size();
            if single_message_payload_size < 0 {
                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                    message: self.message.to_owned(),
                    metadata: self.metadata.to_owned(),
                    payload: PayloadCommandPayloadWithParsed::Batch(Err(
                        FrameParseBatchPayloadError::GetSingleMessagePayloadFailed.into(),
                    )),
                    is_checksum_match,
                }));
            }
            if payload_slice.len() - n_parsed < single_message_payload_size as usize {
                return CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
                    message: self.message.to_owned(),
                    metadata: self.metadata.to_owned(),
                    payload: PayloadCommandPayloadWithParsed::Batch(Err(
                        FrameParseBatchPayloadError::GetSingleMessagePayloadFailed.into(),
                    )),
                    is_checksum_match,
                }));
            }
            let single_message_payload =
                payload_slice[n_parsed..n_parsed + single_message_payload_size as usize].to_vec();
            n_parsed += single_message_payload_size as usize;

            msgs.push((single_message_metadata, single_message_payload));
        }

        CommandWithParsed::Payload(Box::new(PayloadCommandWithParsed {
            message: self.message.to_owned(),
            metadata: self.metadata.to_owned(),
            payload: PayloadCommandPayloadWithParsed::Batch(Ok(msgs)),
            is_checksum_match,
        }))
    }
}
