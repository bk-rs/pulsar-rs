use std::fmt;

use pulsar_binary_protocol_spec::{
    client_channel::AC_Sender, client_channel_messages::ConsumerSendHandlerChannelMessage,
    types::ConsumerId, SubscribeCommand, SuccessCommand,
};

mod get_message;
mod raw_ack;
mod raw_flow;
mod raw_redeliver_unacknowledged_messages;

pub struct AsyncConsumer {
    sender: AC_Sender<ConsumerSendHandlerChannelMessage>,
    subscribe_command: SubscribeCommand,
    success_command: SuccessCommand,
}
impl AsyncConsumer {
    pub(crate) fn new(
        sender: AC_Sender<ConsumerSendHandlerChannelMessage>,
        subscribe_command: SubscribeCommand,
        success_command: SuccessCommand,
    ) -> Self {
        Self {
            sender,
            subscribe_command,
            success_command,
        }
    }

    pub fn get_consumer_id(&self) -> ConsumerId {
        self.subscribe_command.get_consumer_id()
    }
}

impl fmt::Debug for AsyncConsumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncConsumer")
            .field("subscribe_command", &self.subscribe_command)
            .field("success_command", &self.success_command)
            .finish()
    }
}
