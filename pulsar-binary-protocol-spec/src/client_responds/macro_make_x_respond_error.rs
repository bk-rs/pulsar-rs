macro_rules! make_x_respond_error {
    ($name:ident; $($variant:ident $ref:expr),*) => {
        use std::io::Error as IoError;

        use crate::{types::ServerError, frame::{FrameParseError, FrameRenderError}, client_handler::{WriteCommandError, ReadCommandError}};

        paste! {
            #[derive(thiserror::Error, Debug)]
            pub enum [<$name RespondError>] {
                #[error("FrameRenderError {0:?}")]
                FrameRenderError(#[from] FrameRenderError),

                #[error("WriteError {0:?}")]
                WriteError(IoError),

                #[error("ReadError {0:?}")]
                ReadError(IoError),

                #[error("FrameParseError {0:?}")]
                FrameParseError(#[from] FrameParseError),

                $(
                    #[error("{server_error:?} {msg}")]
                    [<SE $variant>] { server_error: ServerError, msg: String },
                )*

                #[error("Unimplemented {server_error:?} {msg}")]
                SEUnimplemented {
                    server_error: ServerError,
                    msg: String,
                },
            }

            impl From<(ServerError, &str)> for [<$name RespondError>] {
                fn from(t: (ServerError, &str)) -> Self {
                    let (se, msg) = t;
                    match se {
                        $(
                            ServerError::$variant => Self::[<SE $variant>] {
                                server_error: se,
                                msg: msg.to_owned(),
                            },
                        )*
                        _ => Self::SEUnimplemented {
                            server_error: se,
                            msg: msg.to_owned(),
                        },
                    }
                }
            }

            impl From<WriteCommandError> for [<$name RespondError>] {
                fn from(err: WriteCommandError) -> Self {
                    match err {
                        WriteCommandError::FrameRenderError(err) => Self::FrameRenderError(err),
                        WriteCommandError::WriteError(err) => Self::WriteError(err),
                    }
                }
            }

            impl From<ReadCommandError> for [<$name RespondError>] {
                fn from(err: ReadCommandError) -> Self {
                    match err {
                        ReadCommandError::ReadError(err) => Self::ReadError(err),
                        ReadCommandError::FrameParseError(err) => Self::FrameParseError(err),
                    }
                }
            }
        }
    };
}
