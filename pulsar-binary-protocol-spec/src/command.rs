use crate::{
    frame::{FrameParseBatchPayloadError, FrameParseSinglePayloadError},
    protos::protobuf::pulsar_api::{BaseCommand, MessageMetadata, SingleMessageMetadata},
    types::AckValidationError,
};

#[derive(Debug, Clone)]
pub enum Command {
    Simple(SimpleCommand),
    Payload(Box<PayloadCommand>),
}

#[derive(Debug, Clone)]
pub enum CommandWithParsed {
    Simple(SimpleCommand),
    Payload(Box<PayloadCommandWithParsed>),
}

// http://pulsar.apache.org/docs/en/develop-binary-protocol/#simple-commands
#[derive(Debug, Clone)]
pub struct SimpleCommand {
    pub message: BaseCommand,
}

// http://pulsar.apache.org/docs/en/develop-binary-protocol/#payload-commands
#[derive(Debug, Clone)]
pub struct PayloadCommand {
    pub message: BaseCommand,
    pub metadata: MessageMetadata,
    pub payload: PayloadCommandPayload,
}

#[derive(Debug, Clone)]
pub enum PayloadCommandPayload {
    Single(Vec<u8>),
    // http://pulsar.apache.org/docs/en/develop-binary-protocol/#batch-messages
    Batch(Vec<(SingleMessageMetadata, Vec<u8>)>),
}

#[derive(Debug, Clone)]
pub struct PayloadCommandWithParsed {
    pub message: BaseCommand,
    pub metadata: MessageMetadata,
    pub payload: PayloadCommandPayloadWithParsed,
    pub is_checksum_match: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum PayloadCommandPayloadWithParsed {
    Single(Result<Vec<u8>, PayloadCommandPayloadErrorWithParsed>),
    Batch(Result<Vec<(SingleMessageMetadata, Vec<u8>)>, PayloadCommandPayloadErrorWithParsed>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PayloadCommandPayloadErrorWithParsed {
    DecompressionError,
    UncompressedSizeCorruption,
    BatchDeSerializeError,
}
impl From<FrameParseSinglePayloadError> for PayloadCommandPayloadErrorWithParsed {
    fn from(err: FrameParseSinglePayloadError) -> Self {
        match err {
            FrameParseSinglePayloadError::CompressionUnsupported { type_code: _ } => {
                Self::DecompressionError
            }
            #[cfg(feature = "with-compression-lz4")]
            FrameParseSinglePayloadError::CompressionLZ4DecompressError(_) => {
                Self::DecompressionError
            }
            #[cfg(feature = "with-compression-zlib")]
            FrameParseSinglePayloadError::CompressionZlibDecompressError(_) => {
                Self::DecompressionError
            }
            FrameParseSinglePayloadError::UncompressedSizeCorruption => {
                Self::UncompressedSizeCorruption
            }
        }
    }
}
impl From<FrameParseBatchPayloadError> for PayloadCommandPayloadErrorWithParsed {
    fn from(err: FrameParseBatchPayloadError) -> Self {
        match err {
            FrameParseBatchPayloadError::CompressionUnsupported { type_code: _ } => {
                Self::DecompressionError
            }
            #[cfg(feature = "with-compression-lz4")]
            FrameParseBatchPayloadError::CompressionLZ4DecompressError(_) => {
                Self::DecompressionError
            }
            #[cfg(feature = "with-compression-zlib")]
            FrameParseBatchPayloadError::CompressionZlibDecompressError(_) => {
                Self::DecompressionError
            }
            FrameParseBatchPayloadError::UncompressedSizeCorruption => {
                Self::UncompressedSizeCorruption
            }
            FrameParseBatchPayloadError::GetSingleMessageMetadataSizeFailed => {
                Self::BatchDeSerializeError
            }
            FrameParseBatchPayloadError::GetSingleMessageMetadataFailed => {
                Self::BatchDeSerializeError
            }
            FrameParseBatchPayloadError::DeserializeSingleMessageMetadataError(_) => {
                Self::BatchDeSerializeError
            }
            FrameParseBatchPayloadError::GetSingleMessagePayloadFailed => {
                Self::BatchDeSerializeError
            }
        }
    }
}
impl From<&PayloadCommandPayloadErrorWithParsed> for AckValidationError {
    fn from(err: &PayloadCommandPayloadErrorWithParsed) -> Self {
        match err {
            PayloadCommandPayloadErrorWithParsed::DecompressionError => Self::DecompressionError,
            PayloadCommandPayloadErrorWithParsed::UncompressedSizeCorruption => {
                Self::UncompressedSizeCorruption
            }
            PayloadCommandPayloadErrorWithParsed::BatchDeSerializeError => {
                Self::BatchDeSerializeError
            }
        }
    }
}
