use crate::commands::RedeliverUnacknowledgedMessagesCommand;

use super::HalfRequest;

pub struct ConsumerRedeliverUnacknowledgedMessagesHalfRequest {}
impl HalfRequest for ConsumerRedeliverUnacknowledgedMessagesHalfRequest {
    type Request = RedeliverUnacknowledgedMessagesCommand;
    type Error = ConsumerRedeliverUnacknowledgedMessagesHalfRequestError;
}

make_x_half_request_error!(
    ConsumerRedeliverUnacknowledgedMessages;
);
