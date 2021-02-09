use crate::{
    client_channel::FC_Sender,
    client_channel_messages::{
        handler_reply_consumer_channel_message::HandlerReplyConsumerAckChannelMessage,
        handler_reply_producer_channel_message::HandlerReplyProducerSendChannelMessage,
        handler_reply_session_channel_message::{
            HandlerReplySessionCreateConsumerChannelMessage,
            HandlerReplySessionCreateProducerChannelMessage,
        },
    },
    client_responds::{
        ConsumerAckRespond, ProducerSendRespond, Respond, SessionCreateConsumerRespond,
        SessionCreateProducerRespond,
    },
};

#[derive(Debug)]
pub enum OnResponded {
    SessionCreateProducer(
        <SessionCreateProducerRespond as Respond>::Request,
        FC_Sender<HandlerReplySessionCreateProducerChannelMessage>,
        Result<
            <SessionCreateProducerRespond as Respond>::Response,
            <SessionCreateProducerRespond as Respond>::Error,
        >,
    ),
    SessionCreateConsumer(
        <SessionCreateConsumerRespond as Respond>::Request,
        FC_Sender<HandlerReplySessionCreateConsumerChannelMessage>,
        Result<
            <SessionCreateConsumerRespond as Respond>::Response,
            <SessionCreateConsumerRespond as Respond>::Error,
        >,
    ),
    ProducerSend(
        FC_Sender<HandlerReplyProducerSendChannelMessage>,
        Result<<ProducerSendRespond as Respond>::Response, <ProducerSendRespond as Respond>::Error>,
    ),
    ConsumerAck(
        FC_Sender<HandlerReplyConsumerAckChannelMessage>,
        Result<<ConsumerAckRespond as Respond>::Response, <ConsumerAckRespond as Respond>::Error>,
    ),
}
