use crate::{
    protos::protobuf::pulsar_api::CommandError,
    types::{RequestId, ServerError},
};

#[derive(Clone, Debug)]
pub struct ErrorCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandError,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandError,
}
impl ErrorCommand {
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
