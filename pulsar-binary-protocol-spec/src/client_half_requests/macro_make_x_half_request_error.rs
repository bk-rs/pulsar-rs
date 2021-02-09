macro_rules! make_x_half_request_error {
    ($name:ident; $($variant:ident $ref:expr),*) => {
        use std::io::Error as IoError;

        use crate::{client_handler::WriteCommandError, frame::FrameRenderError};

        paste! {
            #[derive(thiserror::Error, Debug)]
            pub enum [<$name HalfRequestError>] {
                #[error("FrameRenderError {0:?}")]
                FrameRenderError(#[from] FrameRenderError),

                #[error("WriteError {0:?}")]
                WriteError(IoError),
            }

            impl From<WriteCommandError> for [<$name HalfRequestError>] {
                fn from(err: WriteCommandError) -> Self {
                    match err {
                        WriteCommandError::FrameRenderError(err) => Self::FrameRenderError(err),
                        WriteCommandError::WriteError(err) => Self::WriteError(err),
                    }
                }
            }
        }
    };
}
