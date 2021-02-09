use protobuf::SingularPtrField;

use crate::{
    command::{Command, SimpleCommand},
    protos::{
        protobuf::pulsar_api::{BaseCommand, BaseCommand_Type as Type, CommandProducer},
        utils::convert_tuple_slice_to_key_value_vector,
    },
    types::{ProducerId, RequestId},
};

#[derive(Clone, Debug)]
pub struct ProducerCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandProducer,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandProducer,
}
impl ProducerCommand {
    pub fn new(topic: &str) -> Self {
        let mut inner_command = CommandProducer::new();
        inner_command.set_topic(topic.into());

        Self { inner_command }
    }

    pub fn set_producer_id(&mut self, producer_id: ProducerId) -> &mut Self {
        self.inner_command.set_producer_id(producer_id.into());
        self
    }
    pub fn get_producer_id(&self) -> ProducerId {
        ProducerId::new(self.inner_command.get_producer_id())
    }
    pub fn set_request_id(&mut self, request_id: RequestId) -> &mut Self {
        self.inner_command.set_request_id(request_id.into());
        self
    }
    pub fn get_request_id(&self) -> RequestId {
        RequestId::new(self.inner_command.get_request_id())
    }

    pub fn set_producer_name(&mut self, producer_name: &str) -> &mut Self {
        self.inner_command.set_producer_name(producer_name.into());
        self
    }

    pub fn append_metadata(&mut self, metadata: &[(&str, &str)]) -> &mut Self {
        for kv in convert_tuple_slice_to_key_value_vector(metadata) {
            self.inner_command.metadata.push(kv);
        }

        self
    }
    pub fn clear_metadata(&mut self) -> &mut Self {
        self.inner_command.metadata.clear();
        self
    }
}

impl From<&ProducerCommand> for Command {
    fn from(c: &ProducerCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::PRODUCER);
        base_command.producer = SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
