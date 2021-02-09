#[macro_use]
mod macro_make_x_half_request_error;

pub trait HalfRequest {
    type Request;
    type Error;
}

pub mod consumer_flow_half_request;
pub mod consumer_redeliver_unacknowledged_messages_half_request;

pub use consumer_flow_half_request::{ConsumerFlowHalfRequest, ConsumerFlowHalfRequestError};
pub use consumer_redeliver_unacknowledged_messages_half_request::{
    ConsumerRedeliverUnacknowledgedMessagesHalfRequest,
    ConsumerRedeliverUnacknowledgedMessagesHalfRequestError,
};
