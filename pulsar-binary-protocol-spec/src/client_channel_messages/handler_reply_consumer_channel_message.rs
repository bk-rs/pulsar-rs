use crate::{
    client_half_requests::{
        ConsumerFlowHalfRequest, ConsumerRedeliverUnacknowledgedMessagesHalfRequest, HalfRequest,
    },
    client_responds::{ConsumerAckRespond, Respond},
    commands::MessageCommand,
};

pub type HandlerReplyConsumerFlowChannelMessage =
    Result<(), <ConsumerFlowHalfRequest as HalfRequest>::Error>;
pub type HandlerReplyConsumerGetMessageChannelMessage = Option<MessageCommand>;
pub type HandlerReplyConsumerAckChannelMessage =
    Result<<ConsumerAckRespond as Respond>::Response, <ConsumerAckRespond as Respond>::Error>;
pub type HandlerReplyConsumerRedeliverUnacknowledgedMessagesChannelMessage =
    Result<(), <ConsumerRedeliverUnacknowledgedMessagesHalfRequest as HalfRequest>::Error>;

pub enum HandlerReplyConsumerChannelMessage {
    ReplyFlow(HandlerReplyConsumerFlowChannelMessage),
    ReplyGetMessage(Box<HandlerReplyConsumerGetMessageChannelMessage>),
    ReplyAck(HandlerReplyConsumerAckChannelMessage),
    ReplyRedeliverUnacknowledgedMessages(
        HandlerReplyConsumerRedeliverUnacknowledgedMessagesChannelMessage,
    ),
}
