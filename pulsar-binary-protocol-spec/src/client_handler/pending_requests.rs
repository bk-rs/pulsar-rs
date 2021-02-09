use std::collections::BTreeMap;

use crate::{
    client_channel::FC_Sender,
    client_channel_messages::{
        handler_reply_consumer_channel_message::HandlerReplyConsumerAckChannelMessage,
        handler_reply_session_channel_message::{
            HandlerReplySessionCreateConsumerChannelMessage,
            HandlerReplySessionCreateProducerChannelMessage,
        },
    },
    client_responds::{Respond, SessionCreateConsumerRespond, SessionCreateProducerRespond},
    types::RequestId,
};

pub type PendingRequests = BTreeMap<RequestId, PendingRequestValue>;

pub enum PendingRequestValue {
    SessionCreateProducer(
        <SessionCreateProducerRespond as Respond>::Request,
        FC_Sender<HandlerReplySessionCreateProducerChannelMessage>,
    ),
    SessionCreateConsumer(
        <SessionCreateConsumerRespond as Respond>::Request,
        FC_Sender<HandlerReplySessionCreateConsumerChannelMessage>,
    ),
    ConsumerAck(FC_Sender<HandlerReplyConsumerAckChannelMessage>),
}
