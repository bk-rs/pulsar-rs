use thiserror::Error;

use crate::{
    client_responds::{ConnectRespond, Respond},
    command::CommandWithParsed,
    commands::{MessageCommand, PingCommand, PongCommand},
    protos::protobuf::pulsar_api::{BaseCommand, BaseCommand_Type as Type},
};

mod handle_ack_response;
mod handle_connected;
mod handle_error;
mod handle_message;
mod handle_ping;
mod handle_pong;
mod handle_producer_success;
mod handle_send_error;
mod handle_send_receipt;
mod handle_success;

pub mod errors;
pub mod on_responded;
pub mod pending_messages;
pub mod pending_requests;
pub mod pending_sequences;

pub use errors::{ReadCommandError, WriteCommandError};
pub use on_responded::OnResponded;
pub use pending_messages::PendingMessages;
pub use pending_requests::{PendingRequestValue, PendingRequests};
pub use pending_sequences::{PendingSequenceValue, PendingSequences};

#[derive(Debug)]
pub enum HandlerHandleOutput {
    BrokerPing(PingCommand),
    OnPingResponded(PongCommand),
    OnConnectResponded(
        Result<<ConnectRespond as Respond>::Response, <ConnectRespond as Respond>::Error>,
    ),
    OnResponded(Box<OnResponded>),
    BrokerPushMessage(Box<MessageCommand>),
}

#[derive(Error, Debug)]
pub enum HandlerHandleError {
    #[error("BaseCommandInvalid {0:?}")]
    BaseCommandInvalid(BaseCommand),

    #[error("PendingRequestNotFount {0:?}")]
    PendingRequestNotFount(BaseCommand),
    #[error("PendingRequestMismatch {0:?}")]
    PendingRequestMismatch(BaseCommand),

    #[error("PendingSequenceNotFount {0:?}")]
    PendingSequenceNotFount(BaseCommand),

    #[error("Unsupported {0:?}")]
    Unsupported(BaseCommand),
}

pub fn handle(
    command: &CommandWithParsed,
    pending_requests: &mut PendingRequests,
    pending_sequences: &mut PendingSequences,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    match command {
        CommandWithParsed::Simple(c) => match &c.message.get_field_type() {
            Type::PING => handle_ping::handle_ping(&c.message),
            Type::PONG => handle_pong::handle_pong(&c.message),
            //
            Type::PRODUCER_SUCCESS => {
                handle_producer_success::handle_producer_success(&c.message, pending_requests)
            }
            Type::SUCCESS => handle_success::handle_success(&c.message, pending_requests),
            Type::ERROR => handle_error::handle_error(&c.message, pending_requests),

            //
            Type::SEND_RECEIPT => {
                handle_send_receipt::handle_send_receipt(&c.message, pending_sequences)
            }
            Type::SEND_ERROR => handle_send_error::handle_send_error(&c.message, pending_sequences),

            //
            Type::ACK_RESPONSE => {
                handle_ack_response::handle_ack_response(&c.message, pending_requests)
            }

            //
            _ => Err(HandlerHandleError::Unsupported(c.message.to_owned())),
        },
        CommandWithParsed::Payload(c) => match &c.message.get_field_type() {
            Type::MESSAGE => handle_message::handle_message(
                &c.message,
                &c.metadata,
                &c.payload,
                c.is_checksum_match,
            ),
            _ => Err(HandlerHandleError::Unsupported(c.message.to_owned())),
        },
    }
}

pub fn handle_with_connect(
    command: &CommandWithParsed,
) -> Result<HandlerHandleOutput, HandlerHandleError> {
    match command {
        CommandWithParsed::Simple(c) => match &c.message.get_field_type() {
            Type::CONNECTED => handle_connected::handle_connected(&c.message),
            Type::ERROR => handle_error::handle_error_with_connect(&c.message),
            _ => Err(HandlerHandleError::Unsupported(c.message.to_owned())),
        },
        CommandWithParsed::Payload(c) => Err(HandlerHandleError::Unsupported(c.message.to_owned())),
    }
}
