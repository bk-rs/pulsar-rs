use crate::{
    protos::protobuf::pulsar_api::CommandProducerSuccess,
    types::{ProducerName, RequestId},
};

#[derive(Clone, Debug)]
pub struct ProducerSuccessCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandProducerSuccess,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandProducerSuccess,
}
impl ProducerSuccessCommand {
    pub fn get_request_id(&self) -> RequestId {
        RequestId::new(self.inner_command.get_request_id())
    }

    pub fn get_producer_name(&self) -> ProducerName {
        ProducerName::new(self.inner_command.get_producer_name())
    }
}
