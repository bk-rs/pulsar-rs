use pulsar_binary_protocol_spec::{
    client_channel_messages::{
        handler_reply_consumer_channel_message::HandlerReplyConsumerGetMessageChannelMessage,
        ConsumerSendHandlerChannelMessage,
    },
    futures_channel::oneshot::channel,
    MessageCommand,
};
use thiserror::Error;

use super::AsyncConsumer;

#[derive(Error, Debug)]
pub enum GetMessageError {
    #[error("ConsumerChannelClosed")]
    ConsumerChannelClosed,
    #[error("ChannelClosed")]
    ChannelClosed,
}
impl AsyncConsumer {
    pub async fn get_message(&self) -> Result<Option<MessageCommand>, GetMessageError> {
        let (sender, receiver) = channel::<HandlerReplyConsumerGetMessageChannelMessage>();

        self.sender
            .send(ConsumerSendHandlerChannelMessage::GetMessage(sender))
            .await
            .map_err(|_| GetMessageError::ConsumerChannelClosed)?;

        match receiver.await {
            Ok(message_command) => Ok(message_command),
            Err(_) => Err(GetMessageError::ChannelClosed),
        }
    }
}
