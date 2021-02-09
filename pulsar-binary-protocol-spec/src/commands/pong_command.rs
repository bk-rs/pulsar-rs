use protobuf::SingularPtrField;

use crate::{
    command::{Command, SimpleCommand},
    protos::protobuf::pulsar_api::{BaseCommand, BaseCommand_Type as Type, CommandPong},
};

#[derive(Default, Debug, Clone)]
pub struct PongCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandPong,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandPong,
}
impl PongCommand {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<&PongCommand> for Command {
    fn from(c: &PongCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::PONG);
        base_command.pong = SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
