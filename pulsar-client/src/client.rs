use pulsar_binary_protocol_spec::{
    async_channel::unbounded,
    client_channel_messages::SessionSendHandlerChannelMessage,
    client_handler::{handle_with_connect, HandlerHandleOutput},
    client_responds::ConnectRespondError,
    ConnectCommand,
};
use thiserror::Error;

use crate::session::AsyncSession;

use super::{connection::AsyncConnection, handler::AsyncHandler, AsyncRead, AsyncWrite};

pub struct AsyncClient<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    connection: AsyncConnection<S>,
}
impl<S> AsyncClient<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(connection: AsyncConnection<S>) -> Self {
        Self { connection }
    }
}

#[derive(Error, Debug)]
pub enum RawConnectError {
    #[error("RespondError {0:?}")]
    RespondError(#[from] ConnectRespondError),
    #[error("Unknown {0:?}")]
    Unknown(String),
}

impl<S> AsyncClient<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn raw_connect(
        mut self,
        mut command_connect: ConnectCommand,
    ) -> Result<(AsyncSession, AsyncHandler<S>), RawConnectError> {
        self.connection
            .write_command(&command_connect)
            .await
            .map_err(ConnectRespondError::from)?;
        let commands = self
            .connection
            .try_read_commands(1)
            .await
            .map_err(ConnectRespondError::from)?;

        let commands = commands
            .ok_or_else(|| RawConnectError::Unknown("Not receive any command".to_owned()))?;
        let command = commands
            .first()
            .ok_or_else(|| RawConnectError::Unknown("Not receive any command".to_owned()))?;

        match handle_with_connect(&command) {
            Ok(HandlerHandleOutput::OnConnectResponded(Ok(connected_command))) => {
                if let Some(max_message_size) = connected_command.get_max_message_size() {
                    self.connection
                        .get_mut_frame_renderer()
                        .get_mut_config()
                        .set_max_frame_size(max_message_size);
                    self.connection
                        .get_mut_frame_parser()
                        .get_mut_config()
                        .set_max_frame_size(max_message_size);
                }

                command_connect.hide_auth_data(b"******");

                let (sender, receiver) = unbounded::<SessionSendHandlerChannelMessage>();

                Ok((
                    AsyncSession::new(sender, command_connect, connected_command),
                    AsyncHandler::new(self.connection, receiver),
                ))
            }
            Ok(HandlerHandleOutput::OnConnectResponded(Err(err))) => Err(err.into()),
            Ok(output) => Err(RawConnectError::Unknown(format!(
                "Receive wrong command {:?}",
                output
            ))),
            Err(err) => Err(RawConnectError::Unknown(format!(
                "Receive wrong command {:?}",
                err
            ))),
        }
    }
}
