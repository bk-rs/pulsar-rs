use protobuf::ProtobufEnum as _;

use crate::{
    protos::protobuf::pulsar_api::{CommandConnected, ProtocolVersion as Protobuf_ProtocolVersion},
    types::ProtocolVersion,
};

#[derive(Clone, Debug)]
pub struct ConnectedCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandConnected,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandConnected,
}
impl ConnectedCommand {
    pub fn get_server_version(&self) -> &str {
        self.inner_command.get_server_version()
    }

    pub fn get_protocol_version(&self) -> Option<ProtocolVersion> {
        Protobuf_ProtocolVersion::from_i32(self.inner_command.get_protocol_version())
            .map(Into::into)
    }

    pub fn get_max_message_size(&self) -> Option<u32> {
        if self.inner_command.has_max_message_size() {
            Some(self.inner_command.get_max_message_size() as u32)
        } else {
            None
        }
    }
}
