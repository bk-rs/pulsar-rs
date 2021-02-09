use crate::protos::protobuf::pulsar_api::CommandAck_ValidationError as Protobuf_ValidationError;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AckValidationError {
    UncompressedSizeCorruption,
    DecompressionError,
    ChecksumMismatch,
    BatchDeSerializeError,
    DecryptionError,
}

impl From<AckValidationError> for Protobuf_ValidationError {
    fn from(ave: AckValidationError) -> Self {
        match ave {
            AckValidationError::UncompressedSizeCorruption => {
                Protobuf_ValidationError::UncompressedSizeCorruption
            }
            AckValidationError::DecompressionError => Protobuf_ValidationError::DecompressionError,
            AckValidationError::ChecksumMismatch => Protobuf_ValidationError::ChecksumMismatch,
            AckValidationError::BatchDeSerializeError => {
                Protobuf_ValidationError::BatchDeSerializeError
            }
            AckValidationError::DecryptionError => Protobuf_ValidationError::DecryptionError,
        }
    }
}
