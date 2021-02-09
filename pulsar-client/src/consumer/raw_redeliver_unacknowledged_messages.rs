use pulsar_binary_protocol_spec::{
    client_channel_messages::{
        handler_reply_consumer_channel_message::HandlerReplyConsumerRedeliverUnacknowledgedMessagesChannelMessage,
        ConsumerSendHandlerChannelMessage,
    },
    client_half_requests::ConsumerRedeliverUnacknowledgedMessagesHalfRequestError,
    futures_channel::oneshot::channel,
    RedeliverUnacknowledgedMessagesCommand,
};
use thiserror::Error;

use super::AsyncConsumer;

#[derive(Error, Debug)]
pub enum RawRedeliverUnacknowledgedMessagesError {
    #[error("ConsumerChannelClosed")]
    ConsumerChannelClosed,
    #[error("RespondError {0:?}")]
    RespondError(ConsumerRedeliverUnacknowledgedMessagesHalfRequestError),
    #[error("ChannelClosed")]
    ChannelClosed,
}
impl AsyncConsumer {
    pub async fn raw_redeliver_unacknowledged_messages(
        &self,
        redeliver_unacknowledged_messages_command: RedeliverUnacknowledgedMessagesCommand,
    ) -> Result<(), RawRedeliverUnacknowledgedMessagesError> {
        let (sender, receiver) =
            channel::<HandlerReplyConsumerRedeliverUnacknowledgedMessagesChannelMessage>();

        self.sender
            .send(
                ConsumerSendHandlerChannelMessage::RedeliverUnacknowledgedMessages(
                    redeliver_unacknowledged_messages_command,
                    sender,
                ),
            )
            .await
            .map_err(|_| RawRedeliverUnacknowledgedMessagesError::ConsumerChannelClosed)?;

        match receiver.await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(RawRedeliverUnacknowledgedMessagesError::RespondError(err)),
            Err(_) => Err(RawRedeliverUnacknowledgedMessagesError::ChannelClosed),
        }
    }
}
