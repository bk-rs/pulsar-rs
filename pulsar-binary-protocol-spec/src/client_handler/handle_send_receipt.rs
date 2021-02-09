use crate::{commands::SendReceiptCommand, protos::protobuf::pulsar_api::BaseCommand};

use super::{HandlerHandleError, HandlerHandleOutput, OnResponded, PendingSequences};

pub(super) fn handle_send_receipt(
    base_command: &BaseCommand,
    pending_sequences: &mut PendingSequences,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    if let Some(c) = base_command.send_receipt.as_ref() {
        let c = SendReceiptCommand {
            inner_command: c.to_owned(),
        };
        if let Some(pending_sequence) = pending_sequences.remove(&c.get_sequence_id()) {
            Ok(HandlerHandleOutput::OnResponded(Box::new(
                OnResponded::ProducerSend(pending_sequence, Ok(c)),
            )))
        } else {
            Err(HandlerHandleError::PendingSequenceNotFount(
                base_command.to_owned(),
            ))
        }
    } else {
        Err(HandlerHandleError::BaseCommandInvalid(
            base_command.to_owned(),
        ))
    }
}
