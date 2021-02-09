pub mod parser;
pub mod renderer;

const MAX_FRAME_SIZE_DEFAULT: u32 = 5 * 1024 * 1024;
const MAGIC_NUMBER: u16 = 0x0e01u16;

pub use parser::{
    FrameParseBatchPayloadError, FrameParseError, FrameParseOutput, FrameParseSinglePayloadError,
    FrameParser,
};
pub use renderer::{FrameRenderError, FrameRenderer};

#[cfg(test)]
mod tests;
