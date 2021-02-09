use protobuf::SingularPtrField;

use crate::{
    command::{Command, SimpleCommand},
    protos::protobuf::pulsar_api::{BaseCommand, BaseCommand_Type as Type, CommandPing},
};

#[derive(Default, Debug, Clone)]
pub struct PingCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandPing,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandPing,
}
impl PingCommand {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<&PingCommand> for Command {
    fn from(c: &PingCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::PING);
        base_command.ping = SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
