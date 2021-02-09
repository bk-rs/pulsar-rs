use std::io::Error as IoError;

use crate::frame::{FrameParseError, FrameRenderError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteCommandError {
    #[error("FrameRenderError {0:?}")]
    FrameRenderError(#[from] FrameRenderError),
    #[error("WriteError {0:?}")]
    WriteError(#[from] IoError),
}
#[derive(Error, Debug)]
pub enum ReadCommandError {
    #[error("ReadError {0:?}")]
    ReadError(#[from] IoError),
    #[error("FrameParseError {0:?}")]
    FrameParseError(#[from] FrameParseError),
}
