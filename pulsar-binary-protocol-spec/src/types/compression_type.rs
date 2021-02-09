use crate::protos::protobuf::pulsar_api::CompressionType as Protobuf_CompressionType;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CompressionType {
    NONE,
    #[cfg(feature = "with-compression-lz4")]
    LZ4,
    #[cfg(feature = "with-compression-zlib")]
    ZLIB,
}

impl From<CompressionType> for Protobuf_CompressionType {
    fn from(ct: CompressionType) -> Self {
        match ct {
            CompressionType::NONE => Protobuf_CompressionType::NONE,
            #[cfg(feature = "with-compression-lz4")]
            CompressionType::LZ4 => Protobuf_CompressionType::LZ4,
            #[cfg(feature = "with-compression-zlib")]
            CompressionType::ZLIB => Protobuf_CompressionType::ZLIB,
        }
    }
}

impl std::convert::TryFrom<Protobuf_CompressionType> for CompressionType {
    type Error = ();
    fn try_from(ct: Protobuf_CompressionType) -> Result<Self, Self::Error> {
        match ct {
            Protobuf_CompressionType::NONE => Ok(Self::NONE),
            #[cfg(feature = "with-compression-lz4")]
            Protobuf_CompressionType::LZ4 => Ok(Self::LZ4),
            #[cfg(feature = "with-compression-zlib")]
            Protobuf_CompressionType::ZLIB => Ok(Self::ZLIB),
            _ => Err(()),
        }
    }
}
