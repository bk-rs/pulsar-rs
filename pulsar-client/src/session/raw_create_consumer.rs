use pulsar_binary_protocol_spec::{
    client_channel_messages::{
        handler_reply_session_channel_message::HandlerReplySessionCreateConsumerChannelMessage,
        SessionSendHandlerChannelMessage,
    },
    client_responds::SessionCreateConsumerRespondError,
    futures_channel::oneshot::channel,
    SubscribeCommand,
};
use thiserror::Error;

use crate::consumer::AsyncConsumer;

use super::AsyncSession;

#[derive(Error, Debug)]
pub enum RawCreateConsumerError {
    #[error("SessionChannelClosed")]
    SessionChannelClosed,
    #[error("RespondError {0:?}")]
    RespondError(SessionCreateConsumerRespondError),
    #[error("ChannelClosed")]
    ChannelClosed,
}
impl AsyncSession {
    pub async fn raw_create_consumer(
        &self,
        subscribe_command: SubscribeCommand,
    ) -> Result<AsyncConsumer, RawCreateConsumerError> {
        let (sender, receiver) = channel::<HandlerReplySessionCreateConsumerChannelMessage>();

        self.sender
            .send(SessionSendHandlerChannelMessage::CreateConsumer(
                subscribe_command,
                sender,
            ))
            .await
            .map_err(|_| RawCreateConsumerError::SessionChannelClosed)?;

        match receiver.await {
            Ok(Ok((subscribe_command, success_command, s))) => {
                Ok(AsyncConsumer::new(s, subscribe_command, success_command))
            }
            Ok(Err(err)) => Err(RawCreateConsumerError::RespondError(err)),
            Err(_) => Err(RawCreateConsumerError::ChannelClosed),
        }
    }
}
