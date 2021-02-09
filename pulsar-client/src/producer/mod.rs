use std::fmt;

use pulsar_binary_protocol_spec::{
    client_channel::AC_Sender,
    client_channel_messages::ProducerSendHandlerChannelMessage,
    types::{ProducerId, ProducerName, SequenceId, SequenceIdBuilder},
    ProducerCommand, ProducerSuccessCommand,
};

mod raw_send;

pub struct AsyncProducer {
    sender: AC_Sender<ProducerSendHandlerChannelMessage>,
    producer_command: ProducerCommand,
    producer_success_command: ProducerSuccessCommand,
    //
    sequence_id_builder: SequenceIdBuilder,
}
impl AsyncProducer {
    pub(crate) fn new(
        sender: AC_Sender<ProducerSendHandlerChannelMessage>,
        producer_command: ProducerCommand,
        producer_success_command: ProducerSuccessCommand,
    ) -> Self {
        Self {
            sender,
            producer_command,
            producer_success_command,
            sequence_id_builder: SequenceIdBuilder::default(),
        }
    }

    pub fn get_producer_id(&self) -> ProducerId {
        self.producer_command.get_producer_id()
    }

    pub fn get_producer_name(&self) -> ProducerName {
        self.producer_success_command.get_producer_name()
    }

    pub fn next_sequence_id(&self) -> SequenceId {
        self.sequence_id_builder.next()
    }
}

impl fmt::Debug for AsyncProducer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncProducer")
            .field("producer_command", &self.producer_command)
            .field("producer_success_command", &self.producer_success_command)
            .finish()
    }
}
