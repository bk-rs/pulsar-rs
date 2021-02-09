use crate::{protos::protobuf::pulsar_api::CommandSuccess, types::RequestId};

#[derive(Clone, Debug)]
pub struct SuccessCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandSuccess,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandSuccess,
}
impl SuccessCommand {
    pub fn get_request_id(&self) -> RequestId {
        RequestId::new(self.inner_command.get_request_id())
    }
}
