use crate::{
    client_channel::FC_Sender,
    client_handler::PendingSequenceValue,
    client_responds::{ProducerSendRespond, Respond},
    command::Command,
    types::{ProducerId, ProducerName, SequenceId},
};

use super::handler_reply_producer_channel_message::HandlerReplyProducerSendChannelMessage;

pub enum ProducerSendHandlerChannelMessage {
    Send(
        <ProducerSendRespond as Respond>::Request,
        FC_Sender<HandlerReplyProducerSendChannelMessage>,
    ),
}

impl ProducerSendHandlerChannelMessage {
    pub fn into_group(
        self,
        producer_id: ProducerId,
        producer_name: ProducerName,
    ) -> ProducerSendHandlerChannelMessageGroup {
        match self {
            Self::Send(mut c, s) => {
                c.set_producer_id(producer_id);
                c.set_producer_name(producer_name);
                let command = Command::from(&c);

                ProducerSendHandlerChannelMessageGroup::PendingSequence(
                    c.get_sequence_id(),
                    s,
                    command,
                )
            }
        }
    }
}

pub enum ProducerSendHandlerChannelMessageGroup {
    PendingSequence(SequenceId, PendingSequenceValue, Command),
}
