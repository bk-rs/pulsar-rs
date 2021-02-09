use crate::{commands::PingCommand, protos::protobuf::pulsar_api::BaseCommand};

use super::{HandlerHandleError, HandlerHandleOutput};

pub(super) fn handle_ping(
    base_command: &BaseCommand,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.ping.as_ref() {
        let c = PingCommand {
            inner_command: c.to_owned(),
        };
        Ok(HandlerHandleOutput::BrokerPing(c))
    } else {
        Err(HandlerHandleError::BaseCommandInvalid(
            base_command.to_owned(),
        ))
    }
}
