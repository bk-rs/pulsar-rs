use std::io::{self, Cursor, Write};

use lz4::{Decoder, EncoderBuilder};

pub fn compress(slice: &[u8], w: &mut Vec<u8>) -> io::Result<()> {
    let mut encoder = EncoderBuilder::new().build(w)?;
    encoder.write_all(slice)?;
    let (_, ret) = encoder.finish();
    ret
}

pub fn decompress(slice: &[u8], w: &mut Vec<u8>) -> io::Result<()> {
    let mut decoder = Decoder::new(Cursor::new(slice.to_vec()))?;
    io::copy(&mut decoder, w)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn decompress_smallest() -> Result<(), Box<dyn error::Error>> {
        // https://github.com/10XGenomics/lz4-rs/blob/59cc0258fa4aac51817c27a0547f7db9215b918a/src/decoder.rs#L128
        const END_MARK: [u8; 4] = [0x9f, 0x77, 0x22, 0x71];

        // https://github.com/10XGenomics/lz4-rs/blob/59cc0258fa4aac51817c27a0547f7db9215b918a/src/decoder.rs#L213
        let mut bytes = b"\x04\x22\x4d\x18\x40\x40\xc0\x00\x00\x00\x00".to_vec();
        bytes.write(&END_MARK).unwrap();

        let mut buf = Vec::new();
        decompress(&bytes[..], &mut buf)?;
        assert_eq!(buf, Vec::new());

        Ok(())
    }

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
