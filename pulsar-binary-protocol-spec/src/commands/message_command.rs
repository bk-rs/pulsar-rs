use crate::{
    command::{PayloadCommandPayloadErrorWithParsed, PayloadCommandPayloadWithParsed},
    protos::protobuf::pulsar_api::{CommandMessage, MessageMetadata as Protobuf_MessageMetadata},
    types::{ConsumerId, MessageIdData, MessageMetadata, SingleMessageMetadata},
};

#[derive(Clone, Debug)]
pub struct MessageCommand {
    #[cfg(feature = "with-hacking-commands")]
    pub inner_command: CommandMessage,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner_command: CommandMessage,

    #[cfg(feature = "with-hacking-commands")]
    pub message_metadata: Protobuf_MessageMetadata,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) message_metadata: Protobuf_MessageMetadata,

    #[cfg(feature = "with-hacking-commands")]
    pub payload: PayloadCommandPayloadWithParsed,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) payload: PayloadCommandPayloadWithParsed,

    pub(crate) is_checksum_match: Option<bool>,
}
impl MessageCommand {
    pub fn get_consumer_id(&self) -> ConsumerId {
        ConsumerId::new(self.inner_command.get_consumer_id())
    }

    pub fn get_message_id(&self) -> Option<MessageIdData> {
        self.inner_command.message_id.as_ref().map(Into::into)
    }

    pub fn get_message_metadata(&self) -> MessageMetadata<'_> {
        (&self.message_metadata).into()
    }

    pub fn get_payload(&self) -> MessageCommandPayload<'_> {
        (&self.payload).into()
    }

    pub fn get_is_checksum_mismatch(&self) -> Option<bool> {
        self.is_checksum_match.map(|x| !x)
    }
}

#[derive(Debug)]
pub enum MessageCommandPayload<'a> {
    Single(Result<&'a [u8], &'a PayloadCommandPayloadErrorWithParsed>),
    Batch(
        Result<
            Vec<(SingleMessageMetadata<'a>, &'a Vec<u8>)>,
            &'a PayloadCommandPayloadErrorWithParsed,
        >,
    ),
}
impl<'a> From<&'a PayloadCommandPayloadWithParsed> for MessageCommandPayload<'a> {
    fn from(pcpwp: &'a PayloadCommandPayloadWithParsed) -> Self {
        match pcpwp {
            PayloadCommandPayloadWithParsed::Single(Ok(bytes)) => Self::Single(Ok(bytes)),
            PayloadCommandPayloadWithParsed::Single(Err(err)) => Self::Single(Err(err)),
            PayloadCommandPayloadWithParsed::Batch(Ok(arr)) => Self::Batch(Ok(arr
                .iter()
                .map(|(smm, bytes)| (SingleMessageMetadata::from(smm), bytes))
                .collect::<Vec<_>>())),
            PayloadCommandPayloadWithParsed::Batch(Err(err)) => Self::Batch(Err(err)),
        }
    }
}
