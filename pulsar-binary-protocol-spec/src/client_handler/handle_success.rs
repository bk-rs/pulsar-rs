use crate::{commands::SuccessCommand, protos::protobuf::pulsar_api::BaseCommand};

use super::{
    HandlerHandleError, HandlerHandleOutput, OnResponded, PendingRequestValue, PendingRequests,
};

pub(super) fn handle_success(
    base_command: &BaseCommand,
    pending_requests: &mut PendingRequests,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.success.as_ref() {
        let c = SuccessCommand {
            inner_command: c.to_owned(),
        };
        if let Some(pending_request) = pending_requests.remove(&c.get_request_id()) {
            match pending_request {
                PendingRequestValue::SessionCreateConsumer(subscribe_command, s) => {
                    Ok(HandlerHandleOutput::OnResponded(Box::new(
                        OnResponded::SessionCreateConsumer(subscribe_command, s, Ok(c)),
                    )))
                }
                _ => Err(HandlerHandleError::PendingRequestMismatch(
                    base_command.to_owned(),
                )),
            }
        } else {
            Err(HandlerHandleError::PendingRequestNotFount(
                base_command.to_owned(),
            ))
        }
    } else {
        Err(HandlerHandleError::BaseCommandInvalid(
            base_command.to_owned(),
        ))
    }
}
