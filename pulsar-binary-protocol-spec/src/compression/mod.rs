#[cfg(feature = "with-compression-lz4")]
pub mod lz4;

#[cfg(feature = "with-compression-zlib")]
pub mod zlib;
