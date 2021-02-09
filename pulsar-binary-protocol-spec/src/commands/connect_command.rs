use protobuf::{ProtobufEnum as _, SingularPtrField};

use crate::{
    command::{Command, SimpleCommand},
    protos::protobuf::pulsar_api::{
        BaseCommand, BaseCommand_Type as Type, CommandConnect,
        ProtocolVersion as Protobuf_ProtocolVersion,
    },
    types::ProtocolVersion,
};

#[derive(Clone, Debug)]
pub struct ConnectCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandConnect,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandConnect,
}
impl ConnectCommand {
    pub fn new(client_version: &str) -> Self {
        let mut inner_command = CommandConnect::new();
        inner_command.set_client_version(client_version.into());

        Self { inner_command }
    }

    pub fn set_auth(&mut self, method_name: &str, data: &[u8]) -> &mut Self {
        self.inner_command.set_auth_method_name(method_name.into());
        self.inner_command.set_auth_data(data.into());
        self
    }

    pub fn set_protocol_version(&mut self, protocol_version: ProtocolVersion) -> &mut Self {
        self.inner_command
            .set_protocol_version(Protobuf_ProtocolVersion::from(protocol_version).value());
        self
    }

    pub fn hide_auth_data(&mut self, value: &[u8]) {
        self.inner_command.set_auth_data(value.into());
    }
}

impl From<&ConnectCommand> for Command {
    fn from(c: &ConnectCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::CONNECT);
        base_command.connect = SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
