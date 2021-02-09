use crate::client_responds::{ProducerSendRespond, Respond};

pub type HandlerReplyProducerSendChannelMessage =
    Result<<ProducerSendRespond as Respond>::Response, <ProducerSendRespond as Respond>::Error>;

pub enum HandlerReplyProducerChannelMessage {
    ReplySend(HandlerReplyProducerSendChannelMessage),
}
