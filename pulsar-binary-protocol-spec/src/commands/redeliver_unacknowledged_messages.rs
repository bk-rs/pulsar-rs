use protobuf::SingularPtrField;

use crate::{
    command::{Command, SimpleCommand},
    protos::protobuf::pulsar_api::{
        BaseCommand, BaseCommand_Type as Type, CommandRedeliverUnacknowledgedMessages,
    },
    types::{ConsumerId, MessageIdData},
};

#[derive(Clone, Debug)]
pub struct RedeliverUnacknowledgedMessagesCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandRedeliverUnacknowledgedMessages,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandRedeliverUnacknowledgedMessages,
}
impl RedeliverUnacknowledgedMessagesCommand {
    pub fn new(message_ids: &[MessageIdData]) -> Self {
        let mut inner_command = CommandRedeliverUnacknowledgedMessages::new();
        for message_id_data in message_ids
            .iter()
            .map(|x| x.inner.to_owned())
            .collect::<Vec<_>>()
        {
            inner_command.message_ids.push(message_id_data);
        }

        Self { inner_command }
    }

    pub fn set_consumer_id(&mut self, consumer_id: ConsumerId) -> &mut Self {
        self.inner_command.set_consumer_id(consumer_id.into());
        self
    }
}

impl From<&RedeliverUnacknowledgedMessagesCommand> for Command {
    fn from(c: &RedeliverUnacknowledgedMessagesCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::REDELIVER_UNACKNOWLEDGED_MESSAGES);
        base_command.redeliverUnacknowledgedMessages =
            SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
