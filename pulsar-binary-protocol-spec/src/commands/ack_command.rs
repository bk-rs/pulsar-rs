use protobuf::SingularPtrField;

use crate::{
    command::{Command, SimpleCommand},
    protos::protobuf::pulsar_api::{BaseCommand, BaseCommand_Type as Type, CommandAck},
    types::{AckType, AckValidationError, ConsumerId, MessageIdData, RequestId},
};

#[derive(Clone, Debug)]
pub struct AckCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandAck,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandAck,
}
impl AckCommand {
    pub fn individual(
        message_ids: &[MessageIdData],
        validation_error: impl Into<Option<AckValidationError>>,
    ) -> Self {
        let mut inner_command = CommandAck::new();
        inner_command.set_ack_type(AckType::Individual.into());
        for message_id_data in message_ids
            .iter()
            .map(|x| x.inner.to_owned())
            .collect::<Vec<_>>()
        {
            inner_command.message_id.push(message_id_data);
        }

        if let Some(validation_error) = validation_error.into() {
            inner_command.set_validation_error(validation_error.into());
        }

        Self { inner_command }
    }

    pub fn set_consumer_id(&mut self, consumer_id: ConsumerId) -> &mut Self {
        self.inner_command.set_consumer_id(consumer_id.into());
        self
    }

    pub fn set_request_id(&mut self, request_id: RequestId) -> &mut Self {
        self.inner_command.set_request_id(request_id.into());
        self
    }
    pub fn get_request_id(&self) -> RequestId {
        RequestId::new(self.inner_command.get_request_id())
    }
}

impl From<&AckCommand> for Command {
    fn from(c: &AckCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::ACK);
        base_command.ack = SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
