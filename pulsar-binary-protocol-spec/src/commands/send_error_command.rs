use crate::{
    protos::protobuf::pulsar_api::CommandSendError,
    types::{ProducerId, SequenceId, ServerError},
};

#[derive(Clone, Debug)]
pub struct SendErrorCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandSendError,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandSendError,
}
impl SendErrorCommand {
    pub fn get_producer_id(&self) -> ProducerId {
        ProducerId::new(self.inner_command.get_producer_id())
    }

    pub fn get_sequence_id(&self) -> SequenceId {
        SequenceId::new(self.inner_command.get_sequence_id())
    }

    pub fn get_error(&self) -> ServerError {
        self.inner_command.get_error().into()
    }

    pub fn get_message(&self) -> &str {
        self.inner_command.get_message()
    }
}
