use crate::{
    client_channel::AC_Sender,
    client_responds::{Respond, SessionCreateConsumerRespond, SessionCreateProducerRespond},
};

use super::{ConsumerSendHandlerChannelMessage, ProducerSendHandlerChannelMessage};

pub type HandlerReplySessionCreateProducerChannelMessage = Result<
    (
        <SessionCreateProducerRespond as Respond>::Request,
        <SessionCreateProducerRespond as Respond>::Response,
        AC_Sender<ProducerSendHandlerChannelMessage>,
    ),
    <SessionCreateProducerRespond as Respond>::Error,
>;

pub type HandlerReplySessionCreateConsumerChannelMessage = Result<
    (
        <SessionCreateConsumerRespond as Respond>::Request,
        <SessionCreateConsumerRespond as Respond>::Response,
        AC_Sender<ConsumerSendHandlerChannelMessage>,
    ),
    <SessionCreateConsumerRespond as Respond>::Error,
>;

pub enum HandlerReplySessionChannelMessage {
    ReplyCreateProducer(HandlerReplySessionCreateProducerChannelMessage),
    ReplyCreateConsumer(HandlerReplySessionCreateConsumerChannelMessage),
}
