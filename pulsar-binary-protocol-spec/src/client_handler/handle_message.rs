use crate::{
    command::PayloadCommandPayloadWithParsed,
    commands::MessageCommand,
    protos::protobuf::pulsar_api::{BaseCommand, MessageMetadata},
};

use super::{HandlerHandleError, HandlerHandleOutput};

pub(super) fn handle_message(
    base_command: &BaseCommand,
    message_metadata: &MessageMetadata,
    payload: &PayloadCommandPayloadWithParsed,
    is_checksum_match: Option<bool>,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.message.as_ref() {
        let c = MessageCommand {
            inner_command: c.to_owned(),
            message_metadata: message_metadata.to_owned(),
            payload: payload.to_owned(),
            is_checksum_match,
        };
        Ok(HandlerHandleOutput::BrokerPushMessage(Box::new(c)))
    } else {
        Err(HandlerHandleError::BaseCommandInvalid(
            base_command.to_owned(),
        ))
    }
}
