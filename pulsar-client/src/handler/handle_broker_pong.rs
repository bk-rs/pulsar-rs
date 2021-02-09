use pulsar_binary_protocol_spec::PongCommand;

use super::HandleError;

pub(super) fn handle_broker_pong(_pong_command: PongCommand) -> Result<(), HandleError> {
    // TODO

    Ok(())
}
