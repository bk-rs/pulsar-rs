use std::{cmp::Ordering, convert::TryFrom};

use crc32c::crc32c;
use protobuf::{Message, ProtobufEnum as _, ProtobufError};
use thiserror::Error;

use crate::{
    command::{Command, PayloadCommandPayload},
    types::CompressionType,
};

use super::{MAGIC_NUMBER, MAX_FRAME_SIZE_DEFAULT};

#[derive(Default, Debug, Clone)]
pub struct FrameRendererConfig {
    max_frame_size: Option<u32>,
    compression_buf_capacity: Option<u32>,
}
impl FrameRendererConfig {
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

#[derive(Error, Debug)]
pub enum FrameRenderError {
    #[error("PayloadTooLarge current:{current} max:{max}")]
    PayloadTooLarge { current: u32, max: u32 },

    #[error("SerializeMessageFailed {0}")]
    SerializeMessageFailed(ProtobufError),

    #[error("CompressionUnsupported type:{type_code}")]
    CompressionUnsupported { type_code: i32 },

    #[cfg(feature = "with-compression-lz4")]
    #[error("CompressionLZ4CompressError {0}")]
    CompressionLZ4CompressError(std::io::Error),

    #[cfg(feature = "with-compression-zlib")]
    #[error("CompressionZlibCompressError {0}")]
    CompressionZlibCompressError(std::io::Error),

    #[error("PayloadUnsupported")]
    PayloadUnsupported,
}

#[derive(Default, Debug, Clone)]
pub struct FrameRenderer {
    config: FrameRendererConfig,
    compression_buf: Vec<u8>,
}
impl FrameRenderer {
    pub fn new() -> Self {
        Self::with_config(Default::default())
    }

    pub fn with_config(config: FrameRendererConfig) -> Self {
        let compression_buf_capacity = config.get_compression_buf_capacity();
        Self {
            config,
            compression_buf: Vec::with_capacity(compression_buf_capacity as usize),
        }
    }

    pub fn get_mut_config(&mut self) -> &mut FrameRendererConfig {
        &mut self.config
    }

    pub fn render<C>(&mut self, command: C, buf: &mut Vec<u8>) -> Result<(), FrameRenderError>
    where
        C: Into<Command>,
    {
        let command: Command = command.into();

        let n_start_with_total_size = buf.len();
        buf.extend_from_slice(&0u32.to_be_bytes()[..]);

        match command {
            Command::Simple(c) => {
                render_protobuf_message(c.message, buf)?;
            }
            Command::Payload(mut c) => {
                let compression_type = c.metadata.get_compression();
                let compression_type =
                    CompressionType::try_from(compression_type).map_err(|_| {
                        FrameRenderError::CompressionUnsupported {
                            type_code: compression_type.value(),
                        }
                    })?;

                render_protobuf_message(c.message, buf)?;

                buf.extend_from_slice(&MAGIC_NUMBER.to_be_bytes()[..]);

                let n_start_with_checksum = buf.len();
                buf.extend_from_slice(&0u32.to_be_bytes()[..]);

                if compression_type != CompressionType::NONE {
                    if let PayloadCommandPayload::Single(bytes) = &c.payload {
                        c.metadata.set_uncompressed_size(bytes.len() as u32)
                    }
                }

                let n_start_with_old_metadata = buf.len();
                render_protobuf_message(c.metadata.to_owned(), buf)?;
                let n_old_metadata = buf.len() - n_start_with_old_metadata;

                match &c.payload {
                    PayloadCommandPayload::Single(bytes) => match compression_type {
                        CompressionType::NONE => {
                            buf.extend_from_slice(&bytes[..]);
                        }
                        #[cfg(feature = "with-compression-lz4")]
                        CompressionType::LZ4 => {
                            self.compression_buf.clear();

                            crate::compression::lz4::compress(
                                &bytes[..],
                                &mut self.compression_buf,
                            )
                            .map_err(FrameRenderError::CompressionLZ4CompressError)?;

                            buf.extend_from_slice(&self.compression_buf[..]);

                            self.compression_buf.clear();
                        }
                        #[cfg(feature = "with-compression-zlib")]
                        CompressionType::ZLIB => {
                            self.compression_buf.clear();

                            crate::compression::zlib::compress(
                                &bytes[..],
                                &mut self.compression_buf,
                            )
                            .map_err(FrameRenderError::CompressionZlibCompressError)?;

                            buf.extend_from_slice(&self.compression_buf[..]);

                            self.compression_buf.clear();
                        }
                    },
                    PayloadCommandPayload::Batch(list) => {
                        let n_start_with_batch = buf.len();
                        for (single_message_metadata, bytes) in list {
                            render_protobuf_message(single_message_metadata.to_owned(), buf)?;
                            buf.extend_from_slice(&bytes[..]);
                        }

                        if compression_type != CompressionType::NONE {
                            let uncompressed_size = buf[n_start_with_batch..].len();

                            match compression_type {
                                CompressionType::NONE => {}
                                #[cfg(feature = "with-compression-lz4")]
                                CompressionType::LZ4 => {
                                    self.compression_buf.clear();

                                    crate::compression::lz4::compress(
                                        &buf[n_start_with_batch..],
                                        &mut self.compression_buf,
                                    )
                                    .map_err(FrameRenderError::CompressionLZ4CompressError)?;

                                    buf.drain(n_start_with_batch..);

                                    buf.extend_from_slice(&self.compression_buf[..]);
                                    self.compression_buf.clear();
                                }
                                #[cfg(feature = "with-compression-zlib")]
                                CompressionType::ZLIB => {
                                    self.compression_buf.clear();

                                    crate::compression::zlib::compress(
                                        &buf[n_start_with_batch..],
                                        &mut self.compression_buf,
                                    )
                                    .map_err(FrameRenderError::CompressionZlibCompressError)?;

                                    buf.drain(n_start_with_batch..);

                                    buf.extend_from_slice(&self.compression_buf[..]);

                                    self.compression_buf.clear();
                                }
                            }

                            c.metadata.set_uncompressed_size(uncompressed_size as u32);
                            let n_start_with_new_metadata = buf.len();
                            render_protobuf_message(c.metadata, buf)?;
                            let n_new_metadata = buf.len() - n_start_with_new_metadata;

                            let slice_new_metadata = buf[n_start_with_new_metadata
                                ..n_start_with_new_metadata + n_new_metadata]
                                .to_vec();
                            buf.drain(n_start_with_new_metadata..);

                            match n_new_metadata.cmp(&n_old_metadata) {
                                Ordering::Less => {
                                    for i in 0..n_old_metadata - n_new_metadata {
                                        buf.remove(n_start_with_old_metadata + n_old_metadata - i);
                                    }
                                }
                                Ordering::Equal => {}
                                Ordering::Greater => {
                                    for i in 0..n_new_metadata - n_old_metadata {
                                        buf.insert(
                                            n_start_with_old_metadata + n_old_metadata + i,
                                            0,
                                        );
                                    }
                                }
                            }

                            buf.splice(
                                n_start_with_old_metadata
                                    ..n_start_with_old_metadata + n_new_metadata,
                                slice_new_metadata.iter().cloned(),
                            );
                        }
                    }
                }

                let n_end = buf.len();
                let checksum = crc32c(&buf[n_start_with_checksum + 4..n_end]);
                buf.splice(
                    n_start_with_checksum..n_start_with_checksum + 4,
                    checksum.to_be_bytes().to_vec(),
                );
            }
        }

        let n_end = buf.len();
        let total_size = (n_end - n_start_with_total_size - 4) as u32;
        buf.splice(
            n_start_with_total_size..n_start_with_total_size + 4,
            total_size.to_be_bytes().to_vec(),
        );

        if total_size > self.config.get_max_frame_size() - 4 {
            return Err(FrameRenderError::PayloadTooLarge {
                current: total_size,
                max: self.config.get_max_frame_size(),
            });
        }

        Ok(())
    }
}

fn render_protobuf_message<M: Message>(
    message: M,
    buf: &mut Vec<u8>,
) -> Result<(), FrameRenderError> {
    let n_start = buf.len();

    buf.extend_from_slice(&0u32.to_be_bytes()[..]);

    message
        .write_to_vec(buf)
        .map_err(FrameRenderError::SerializeMessageFailed)?;

    let n_end = buf.len();

    let size = (n_end - n_start - 4) as u32;

    buf.splice(n_start..n_start + 4, size.to_be_bytes().to_vec());

    Ok(())
}
