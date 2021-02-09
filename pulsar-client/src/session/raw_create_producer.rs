use pulsar_binary_protocol_spec::{
    client_channel_messages::{
        handler_reply_session_channel_message::HandlerReplySessionCreateProducerChannelMessage,
        SessionSendHandlerChannelMessage,
    },
    client_responds::SessionCreateProducerRespondError,
    futures_channel::oneshot::channel,
    ProducerCommand,
};
use thiserror::Error;

use crate::producer::AsyncProducer;

use super::AsyncSession;

#[derive(Error, Debug)]
pub enum RawCreateProducerError {
    #[error("SessionChannelClosed")]
    SessionChannelClosed,
    #[error("RespondError {0:?}")]
    RespondError(SessionCreateProducerRespondError),
    #[error("ChannelClosed")]
    ChannelClosed,
}
impl AsyncSession {
    pub async fn raw_create_producer(
        &self,
        producer_command: ProducerCommand,
    ) -> Result<AsyncProducer, RawCreateProducerError> {
        let (sender, receiver) = channel::<HandlerReplySessionCreateProducerChannelMessage>();

        self.sender
            .send(SessionSendHandlerChannelMessage::CreateProducer(
                producer_command,
                sender,
            ))
            .await
            .map_err(|_| RawCreateProducerError::SessionChannelClosed)?;

        match receiver.await {
            Ok(Ok((producer_command, producer_success_command, s))) => Ok(AsyncProducer::new(
                s,
                producer_command,
                producer_success_command,
            )),
            Ok(Err(err)) => Err(RawCreateProducerError::RespondError(err)),
            Err(_) => Err(RawCreateProducerError::ChannelClosed),
        }
    }
}
