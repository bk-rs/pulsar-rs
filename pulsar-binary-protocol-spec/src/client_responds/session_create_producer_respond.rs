use crate::commands::{ProducerCommand, ProducerSuccessCommand};

use super::Respond;

pub struct SessionCreateProducerRespond {}
impl Respond for SessionCreateProducerRespond {
    type Request = ProducerCommand;
    type Response = ProducerSuccessCommand;
    type Error = SessionCreateProducerRespondError;
}

make_x_respond_error!(
    SessionCreateProducer;
    ServiceNotReady "TODO"
);
