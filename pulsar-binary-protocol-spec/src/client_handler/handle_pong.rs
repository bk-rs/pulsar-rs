use crate::{commands::PongCommand, protos::protobuf::pulsar_api::BaseCommand};

use super::{HandlerHandleError, HandlerHandleOutput};

pub(super) fn handle_pong(
    base_command: &BaseCommand,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.pong.as_ref() {
        let c = PongCommand {
            inner_command: c.to_owned(),
        };
        Ok(HandlerHandleOutput::OnPingResponded(c))
    } else {
        Err(HandlerHandleError::BaseCommandInvalid(
            base_command.to_owned(),
        ))
    }
}
