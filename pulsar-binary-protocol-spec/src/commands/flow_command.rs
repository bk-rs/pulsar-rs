use protobuf::SingularPtrField;

use crate::{
    command::{Command, SimpleCommand},
    protos::protobuf::pulsar_api::{BaseCommand, BaseCommand_Type as Type, CommandFlow},
    types::ConsumerId,
};

#[derive(Clone, Debug)]
pub struct FlowCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandFlow,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandFlow,
}
impl FlowCommand {
    pub fn new(message_permits: u32) -> Self {
        let mut inner_command = CommandFlow::new();
        inner_command.set_messagePermits(message_permits);

        Self { inner_command }
    }

    pub fn set_consumer_id(&mut self, consumer_id: ConsumerId) -> &mut Self {
        self.inner_command.set_consumer_id(consumer_id.into());
        self
    }
}

impl From<&FlowCommand> for Command {
    fn from(c: &FlowCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::FLOW);
        base_command.flow = SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
