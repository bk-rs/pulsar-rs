use protobuf::SingularPtrField;

use crate::{
    command::{Command, SimpleCommand},
    protos::{
        protobuf::pulsar_api::{BaseCommand, BaseCommand_Type as Type, CommandSubscribe},
        utils::convert_tuple_slice_to_key_value_vector,
    },
    types::{ConsumerId, RequestId, SubscribeType},
};

#[derive(Clone, Debug)]
pub struct SubscribeCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandSubscribe,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandSubscribe,
}
impl SubscribeCommand {
    pub fn new(topic: &str, subscription: &str, subscribe_type: SubscribeType) -> Self {
        let mut inner_command = CommandSubscribe::new();
        inner_command.set_topic(topic.into());
        inner_command.set_subscription(subscription.into());
        inner_command.set_subType(subscribe_type.into());

        Self { inner_command }
    }

    pub fn set_consumer_id(&mut self, consumer_id: ConsumerId) -> &mut Self {
        self.inner_command.set_consumer_id(consumer_id.into());
        self
    }
    pub fn get_consumer_id(&self) -> ConsumerId {
        ConsumerId::new(self.inner_command.get_consumer_id())
    }
    pub fn set_request_id(&mut self, request_id: RequestId) -> &mut Self {
        self.inner_command.set_request_id(request_id.into());
        self
    }
    pub fn get_request_id(&self) -> RequestId {
        RequestId::new(self.inner_command.get_request_id())
    }

    pub fn set_consumer_name(&mut self, consumer_name: &str) -> &mut Self {
        self.inner_command.set_consumer_name(consumer_name.into());
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

impl From<&SubscribeCommand> for Command {
    fn from(c: &SubscribeCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::SUBSCRIBE);
        base_command.subscribe = SingularPtrField::some(c.inner_command.to_owned());

        Command::Simple(SimpleCommand {
            message: base_command,
        })
    }
}
