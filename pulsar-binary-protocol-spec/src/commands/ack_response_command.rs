use crate::{
    protos::protobuf::pulsar_api::CommandAckResponse,
    types::{ConsumerId, RequestId, ServerError},
};

#[derive(Clone, Debug)]
pub struct AckResponseCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandAckResponse,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandAckResponse,
}
impl AckResponseCommand {
    pub fn get_consumer_id(&self) -> ConsumerId {
        ConsumerId::new(self.inner_command.get_consumer_id())
    }

    pub fn get_request_id(&self) -> RequestId {
        RequestId::new(self.inner_command.get_request_id())
    }

    pub fn get_error(&self) -> ServerError {
        self.inner_command.get_error().into()
    }

    pub fn get_message(&self) -> &str {
        self.inner_command.get_message()
    }
}
