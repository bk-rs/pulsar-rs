use crate::{commands::ErrorCommand, protos::protobuf::pulsar_api::BaseCommand};

use super::{
    HandlerHandleError, HandlerHandleOutput, OnResponded, PendingRequestValue, PendingRequests,
};

pub(super) fn handle_error(
    base_command: &BaseCommand,
    pending_requests: &mut PendingRequests,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.error.as_ref() {
        let c = ErrorCommand {
            inner_command: c.to_owned(),
        };
        if let Some(pending_request) = pending_requests.remove(&c.get_request_id()) {
            match pending_request {
                PendingRequestValue::SessionCreateProducer(producer_command, s) => Ok(
                    HandlerHandleOutput::OnResponded(Box::new(OnResponded::SessionCreateProducer(
                        producer_command,
                        s,
                        Err((c.get_error(), c.get_message()).into()),
                    ))),
                ),
                PendingRequestValue::SessionCreateConsumer(subscribe_command, s) => Ok(
                    HandlerHandleOutput::OnResponded(Box::new(OnResponded::SessionCreateConsumer(
                        subscribe_command,
                        s,
                        Err((c.get_error(), c.get_message()).into()),
                    ))),
                ),
                PendingRequestValue::ConsumerAck(s) => {
                    Ok(HandlerHandleOutput::OnResponded(Box::new(
                        OnResponded::ConsumerAck(s, Err((c.get_error(), c.get_message()).into())),
                    )))
                }
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

pub(super) fn handle_error_with_connect(
    base_command: &BaseCommand,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.error.as_ref() {
        let c = ErrorCommand {
            inner_command: c.to_owned(),
        };
        Ok(HandlerHandleOutput::OnConnectResponded(Err((
            c.get_error(),
            c.get_message(),
        )
            .into())))
    } else {
        Err(HandlerHandleError::BaseCommandInvalid(
            base_command.to_owned(),
        ))
    }
}
