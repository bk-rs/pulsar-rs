use pulsar_binary_protocol_spec::{
    client_channel_messages::{
        handler_reply_consumer_channel_message::HandlerReplyConsumerAckChannelMessage,
        ConsumerSendHandlerChannelMessage,
    },
    client_responds::ConsumerAckRespondError,
    futures_channel::oneshot::channel,
    AckCommand,
};
use thiserror::Error;

use super::AsyncConsumer;

#[derive(Error, Debug)]
pub enum RawAckError {
    #[error("ConsumerChannelClosed")]
    ConsumerChannelClosed,
    #[error("RespondError {0:?}")]
    RespondError(ConsumerAckRespondError),
    #[error("ChannelClosed")]
    ChannelClosed,
}
impl AsyncConsumer {
    pub async fn raw_ack(&self, ack_command: AckCommand) -> Result<(), RawAckError> {
        let (sender, receiver) = channel::<HandlerReplyConsumerAckChannelMessage>();

        self.sender
            .send(ConsumerSendHandlerChannelMessage::Ack(ack_command, sender))
            .await
            .map_err(|_| RawAckError::ConsumerChannelClosed)?;

        match receiver.await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(RawAckError::RespondError(err)),
            Err(_) => Err(RawAckError::ChannelClosed),
        }
    }
}
