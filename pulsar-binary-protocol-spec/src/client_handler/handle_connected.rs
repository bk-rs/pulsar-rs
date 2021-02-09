use crate::{commands::ConnectedCommand, protos::protobuf::pulsar_api::BaseCommand};

use super::{HandlerHandleError, HandlerHandleOutput};

pub(super) fn handle_connected(
    base_command: &BaseCommand,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.connected.as_ref() {
        let c = ConnectedCommand {
            inner_command: c.to_owned(),
        };
        Ok(HandlerHandleOutput::OnConnectResponded(Ok(c)))
    } else {
        Err(HandlerHandleError::BaseCommandInvalid(
            base_command.to_owned(),
        ))
    }
}
