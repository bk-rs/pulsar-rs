use crate::{
    client_channel::FC_Sender,
    client_half_requests::{
        ConsumerFlowHalfRequest, ConsumerRedeliverUnacknowledgedMessagesHalfRequest, HalfRequest,
    },
    client_handler::PendingRequestValue,
    client_responds::{ConsumerAckRespond, Respond},
    command::Command,
    commands::MessageCommand,
    types::{ConsumerId, RequestId, RequestIdBuilder},
};

use super::handler_reply_consumer_channel_message::{
    HandlerReplyConsumerAckChannelMessage, HandlerReplyConsumerFlowChannelMessage,
    HandlerReplyConsumerGetMessageChannelMessage,
    HandlerReplyConsumerRedeliverUnacknowledgedMessagesChannelMessage,
};

pub enum ConsumerSendHandlerChannelMessage {
    Flow(
        <ConsumerFlowHalfRequest as HalfRequest>::Request,
        FC_Sender<HandlerReplyConsumerFlowChannelMessage>,
    ),
    GetMessage(FC_Sender<HandlerReplyConsumerGetMessageChannelMessage>),
    Ack(
        <ConsumerAckRespond as Respond>::Request,
        FC_Sender<HandlerReplyConsumerAckChannelMessage>,
    ),
    RedeliverUnacknowledgedMessages(
        <ConsumerRedeliverUnacknowledgedMessagesHalfRequest as HalfRequest>::Request,
        FC_Sender<HandlerReplyConsumerRedeliverUnacknowledgedMessagesChannelMessage>,
    ),
}

impl ConsumerSendHandlerChannelMessage {
    pub fn into_group(
        self,
        consumer_id: ConsumerId,
        request_id_builder: &RequestIdBuilder,
    ) -> ConsumerSendHandlerChannelMessageGroup {
        match self {
            Self::Flow(mut c, s) => {
                c.set_consumer_id(consumer_id);

                let command = Command::from(&c);
                ConsumerSendHandlerChannelMessageGroup::Flow(command, s)
            }
            Self::GetMessage(s) => ConsumerSendHandlerChannelMessageGroup::GetMessage(s),
            Self::Ack(mut c, s) => {
                c.set_consumer_id(consumer_id);

                let request_id = request_id_builder.next();
                c.set_request_id(request_id.to_owned());

                let command = Command::from(&c);
                ConsumerSendHandlerChannelMessageGroup::PendingRequest(
                    request_id,
                    PendingRequestValue::ConsumerAck(s),
                    Box::new(command),
                )
            }
            Self::RedeliverUnacknowledgedMessages(mut c, s) => {
                c.set_consumer_id(consumer_id);

                let command = Command::from(&c);
                ConsumerSendHandlerChannelMessageGroup::RedeliverUnacknowledgedMessages(command, s)
            }
        }
    }
}

pub enum ConsumerSendHandlerChannelMessageGroup {
    Flow(
        Command,
        FC_Sender<Result<(), <ConsumerFlowHalfRequest as HalfRequest>::Error>>,
    ),
    GetMessage(FC_Sender<Option<MessageCommand>>),
    PendingRequest(RequestId, PendingRequestValue, Box<Command>),
    RedeliverUnacknowledgedMessages(
        Command,
        FC_Sender<
            Result<(), <ConsumerRedeliverUnacknowledgedMessagesHalfRequest as HalfRequest>::Error>,
        >,
    ),
}
