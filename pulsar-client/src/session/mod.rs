use std::fmt;

use pulsar_binary_protocol_spec::{
    client_channel::AC_Sender, client_channel_messages::SessionSendHandlerChannelMessage,
    ConnectCommand, ConnectedCommand,
};

mod raw_create_consumer;
mod raw_create_producer;

pub struct AsyncSession {
    sender: AC_Sender<SessionSendHandlerChannelMessage>,
    connect_command: ConnectCommand,
    connected_command: ConnectedCommand,
}
impl AsyncSession {
    #[cfg(any(feature = "futures_io", feature = "tokio02_io", feature = "tokio_io",))]
    pub(crate) fn new(
        sender: AC_Sender<SessionSendHandlerChannelMessage>,
        command_connect: ConnectCommand,
        command_connected: ConnectedCommand,
    ) -> Self {
        Self {
            sender,
            connect_command: command_connect,
            connected_command: command_connected,
        }
    }
}

impl fmt::Debug for AsyncSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncSession")
            .field("command_connect", &self.connect_command)
            .field("command_connected", &self.connected_command)
            .finish()
    }
}
