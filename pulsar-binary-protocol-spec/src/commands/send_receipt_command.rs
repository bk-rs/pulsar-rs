use crate::{
    protos::protobuf::pulsar_api::CommandSendReceipt,
    types::{MessageIdData, ProducerId, SequenceId},
};

#[derive(Clone, Debug)]
pub struct SendReceiptCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandSendReceipt,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandSendReceipt,
}
impl SendReceiptCommand {
    pub fn get_producer_id(&self) -> ProducerId {
        ProducerId::new(self.inner_command.get_producer_id())
    }

    pub fn get_sequence_id(&self) -> SequenceId {
        SequenceId::new(self.inner_command.get_sequence_id())
    }

    pub fn get_message_id(&self) -> Option<MessageIdData> {
        self.inner_command.message_id.as_ref().map(Into::into)
    }
}
