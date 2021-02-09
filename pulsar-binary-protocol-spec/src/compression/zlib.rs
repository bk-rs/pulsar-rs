use std::io::{self, Write};

use flate2::{
    write::{ZlibDecoder, ZlibEncoder},
    Compression,
};

pub fn compress(slice: &[u8], w: &mut Vec<u8>) -> io::Result<()> {
    let mut encoder = ZlibEncoder::new(w, Compression::default());
    encoder.write_all(slice)?;
    encoder.finish().map(|_| ())
}

pub fn decompress(slice: &[u8], w: &mut Vec<u8>) -> io::Result<()> {
    let mut decoder = ZlibDecoder::new(w);
    decoder.write_all(slice)?;
    decoder.finish().map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let mut buf = Vec::new();
        compress(b"foo", &mut buf)?;
        let compressed_bytes = buf.to_vec();

        println!("compressed_bytes {:?}", compressed_bytes);

        let mut buf = Vec::new();
        decompress(&compressed_bytes[..], &mut buf)?;
        assert_eq!(buf, b"foo");

        Ok(())
    }
}
