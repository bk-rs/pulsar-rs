use pulsar_binary_protocol_spec::{
    client_channel_messages::{
        handler_reply_producer_channel_message::HandlerReplyProducerSendChannelMessage,
        ProducerSendHandlerChannelMessage,
    },
    client_responds::ProducerSendRespondError,
    futures_channel::oneshot::channel,
    SendCommand, SendReceiptCommand,
};
use thiserror::Error;

use super::AsyncProducer;

#[derive(Error, Debug)]
pub enum RawSendError {
    #[error("ProducerChannelClosed")]
    ProducerChannelClosed,
    #[error("RespondError {0:?}")]
    RespondError(ProducerSendRespondError),
    #[error("ChannelClosed")]
    ChannelClosed,
}
impl AsyncProducer {
    pub async fn raw_send(
        &self,
        send_command: SendCommand,
    ) -> Result<SendReceiptCommand, RawSendError> {
        let (sender, receiver) = channel::<HandlerReplyProducerSendChannelMessage>();

        self.sender
            .send(ProducerSendHandlerChannelMessage::Send(
                send_command,
                sender,
            ))
            .await
            .map_err(|_| RawSendError::ProducerChannelClosed)?;

        match receiver.await {
            Ok(Ok(send_receipt_command)) => Ok(send_receipt_command),
            Ok(Err(err)) => Err(RawSendError::RespondError(err)),
            Err(_) => Err(RawSendError::ChannelClosed),
        }
    }
}
