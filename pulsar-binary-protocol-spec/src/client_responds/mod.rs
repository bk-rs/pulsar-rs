#[macro_use]
mod macro_make_x_respond_error;

pub trait Respond {
    type Request;
    type Response;
    type Error;
}

pub mod connect_respond;
pub mod consumer_ack_respond;
pub mod producer_send_respond;
pub mod session_create_consumer_respond;
pub mod session_create_producer_respond;

pub use connect_respond::{ConnectRespond, ConnectRespondError};
pub use consumer_ack_respond::{ConsumerAckRespond, ConsumerAckRespondError};
pub use producer_send_respond::{ProducerSendRespond, ProducerSendRespondError};
pub use session_create_consumer_respond::{
    SessionCreateConsumerRespond, SessionCreateConsumerRespondError,
};
pub use session_create_producer_respond::{
    SessionCreateProducerRespond, SessionCreateProducerRespondError,
};
