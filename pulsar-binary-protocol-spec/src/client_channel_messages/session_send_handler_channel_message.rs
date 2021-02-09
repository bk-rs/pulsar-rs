use crate::{
    client_channel::FC_Sender,
    client_handler::PendingRequestValue,
    client_responds::{Respond, SessionCreateConsumerRespond, SessionCreateProducerRespond},
    command::Command,
    types::{ConsumerIdBuilder, ProducerIdBuilder, RequestId, RequestIdBuilder},
};

use super::handler_reply_session_channel_message::{
    HandlerReplySessionCreateConsumerChannelMessage,
    HandlerReplySessionCreateProducerChannelMessage,
};

pub enum SessionSendHandlerChannelMessage {
    CreateProducer(
        <SessionCreateProducerRespond as Respond>::Request,
        FC_Sender<HandlerReplySessionCreateProducerChannelMessage>,
    ),
    CreateConsumer(
        <SessionCreateConsumerRespond as Respond>::Request,
        FC_Sender<HandlerReplySessionCreateConsumerChannelMessage>,
    ),
}

impl SessionSendHandlerChannelMessage {
    pub fn into_pending_request_and_command(
        self,
        request_id_builder: &RequestIdBuilder,
        producer_id_builder: &ProducerIdBuilder,
        consumer_id_builder: &ConsumerIdBuilder,
    ) -> ((RequestId, PendingRequestValue), Command) {
        match self {
            Self::CreateProducer(mut c, s) => {
                c.set_producer_id(producer_id_builder.next());

                if c.get_request_id().is_require_set() {
                    c.set_request_id(request_id_builder.next());
                }
                let command = Command::from(&c);
                (
                    (
                        c.get_request_id(),
                        PendingRequestValue::SessionCreateProducer(c, s),
                    ),
                    command,
                )
            }
            Self::CreateConsumer(mut c, s) => {
                c.set_consumer_id(consumer_id_builder.next());

                if c.get_request_id().is_require_set() {
                    c.set_request_id(request_id_builder.next());
                }
                let command = Command::from(&c);
                (
                    (
                        c.get_request_id(),
                        PendingRequestValue::SessionCreateConsumer(c, s),
                    ),
                    command,
                )
            }
        }
    }
}
