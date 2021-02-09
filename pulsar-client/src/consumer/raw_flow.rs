use pulsar_binary_protocol_spec::{
    client_channel_messages::{
        handler_reply_consumer_channel_message::HandlerReplyConsumerFlowChannelMessage,
        ConsumerSendHandlerChannelMessage,
    },
    client_half_requests::ConsumerFlowHalfRequestError,
    futures_channel::oneshot::channel,
    FlowCommand,
};
use thiserror::Error;

use super::AsyncConsumer;

#[derive(Error, Debug)]
pub enum RawFlowError {
    #[error("ConsumerChannelClosed")]
    ConsumerChannelClosed,
    #[error("RespondError {0:?}")]
    RespondError(ConsumerFlowHalfRequestError),
    #[error("ChannelClosed")]
    ChannelClosed,
}
impl AsyncConsumer {
    pub async fn raw_flow(&self, flow_command: FlowCommand) -> Result<(), RawFlowError> {
        let (sender, receiver) = channel::<HandlerReplyConsumerFlowChannelMessage>();

        self.sender
            .send(ConsumerSendHandlerChannelMessage::Flow(
                flow_command,
                sender,
            ))
            .await
            .map_err(|_| RawFlowError::ConsumerChannelClosed)?;

        match receiver.await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(RawFlowError::RespondError(err)),
            Err(_) => Err(RawFlowError::ChannelClosed),
        }
    }
}
