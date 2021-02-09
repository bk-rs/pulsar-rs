use std::time::Duration as StdDuration;

use chrono::{DateTime, Duration, Utc};
use protobuf::SingularPtrField;

use crate::{
    command::{Command, PayloadCommand, PayloadCommandPayload},
    protos::protobuf::pulsar_api::{
        BaseCommand, BaseCommand_Type as Type, CommandSend, MessageMetadata, SingleMessageMetadata,
    },
    types::{CompressionType, MessageProperties, ProducerId, ProducerName, SequenceId},
};

#[derive(Clone, Debug)]
pub struct SendCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandSend,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandSend,

    #[cfg(feature = "with-hacking-commands")]
    pub message_metadata: MessageMetadata,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) message_metadata: MessageMetadata,

    #[cfg(feature = "with-hacking-commands")]
    pub payload: PayloadCommandPayload,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) payload: PayloadCommandPayload,
}
impl SendCommand {
    pub fn single(
        sequence_id: SequenceId,
        properties: impl Into<Option<MessageProperties>>,
        msg: impl AsRef<[u8]>,
        compression: impl Into<Option<CompressionType>>,
    ) -> Self {
        let mut inner_command = CommandSend::new();
        inner_command.set_sequence_id(sequence_id.to_owned().into());

        let mut message_metadata = MessageMetadata::new();
        message_metadata.set_sequence_id(sequence_id.into());
        message_metadata.set_publish_time(Utc::now().timestamp_millis() as u64);

        if let Some(properties) = properties.into() {
            for kv in properties.inner {
                message_metadata.properties.push(kv);
            }
        }

        if let Some(compression) = compression.into() {
            message_metadata.set_compression(compression.into());
        }

        let payload = PayloadCommandPayload::Single(msg.as_ref().to_owned());

        Self {
            inner_command,
            message_metadata,
            payload,
        }
    }

    pub fn batch(
        sequence_id: SequenceId,
        msgs: Vec<(impl Into<MessageProperties>, impl AsRef<[u8]>)>,
        compression: impl Into<Option<CompressionType>>,
    ) -> Self {
        let mut inner_command = CommandSend::new();
        inner_command.set_sequence_id(sequence_id.to_owned().into());
        inner_command.set_num_messages(msgs.len() as i32);

        let mut message_metadata = MessageMetadata::new();
        message_metadata.set_sequence_id(sequence_id.into());
        message_metadata.set_publish_time(Utc::now().timestamp_millis() as u64);
        message_metadata.set_num_messages_in_batch(msgs.len() as i32);

        if let Some(compression) = compression.into() {
            message_metadata.set_compression(compression.into());
        }

        let mut payloads = vec![];
        for (properties, msg) in msgs.into_iter() {
            let mut single_message_metadata = SingleMessageMetadata::new();

            for kv in properties.into().inner {
                single_message_metadata.properties.push(kv);
            }

            single_message_metadata.set_payload_size(msg.as_ref().len() as i32);

            payloads.push((single_message_metadata, msg.as_ref().to_owned()))
        }

        let payload = PayloadCommandPayload::Batch(payloads);

        Self {
            inner_command,
            message_metadata,
            payload,
        }
    }

    pub fn set_producer_id(&mut self, producer_id: ProducerId) -> &mut Self {
        self.inner_command.set_producer_id(producer_id.into());
        self
    }
    pub fn set_producer_name(&mut self, producer_name: ProducerName) -> &mut Self {
        self.message_metadata
            .set_producer_name(producer_name.into());
        self
    }

    pub fn get_sequence_id(&self) -> SequenceId {
        SequenceId::new(self.inner_command.get_sequence_id())
    }

    pub fn set_deliver_at_time(&mut self, dt: DateTime<Utc>) -> &mut Self {
        self.message_metadata
            .set_deliver_at_time(dt.timestamp_millis() as i64);
        self
    }
    pub fn set_deliver_after(&mut self, dur: StdDuration) -> Result<&mut Self, String> {
        let dur = Duration::from_std(dur).map_err(|err| err.to_string())?;

        self.message_metadata
            .set_deliver_at_time((Utc::now() + dur).timestamp_millis() as i64);
        Ok(self)
    }
}

impl From<&SendCommand> for Command {
    fn from(c: &SendCommand) -> Self {
        let mut base_command = BaseCommand::new();
        base_command.set_field_type(Type::SEND);
        base_command.send = SingularPtrField::some(c.inner_command.to_owned());

        Command::Payload(
            PayloadCommand {
                message: base_command,
                metadata: c.message_metadata.to_owned(),
                payload: c.payload.to_owned(),
            }
            .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::SequenceIdBuilder;

    #[test]
    fn single() {
        SendCommand::single(
            SequenceIdBuilder::default().next(),
            None,
            b"foo".to_vec(),
            None,
        );
        SendCommand::single(SequenceIdBuilder::default().next(), None, b"foo", None);
        SendCommand::single(
            SequenceIdBuilder::default().next(),
            None,
            "foo".to_string(),
            None,
        );

        let c = SendCommand::single(
            SequenceIdBuilder::default().next(),
            MessageProperties::from(&[("a", "1")]),
            "foo",
            None,
        );

        match c.payload {
            PayloadCommandPayload::Single(bytes) => assert_eq!(bytes, b"foo"),
            _ => assert!(false),
        }
    }
}
