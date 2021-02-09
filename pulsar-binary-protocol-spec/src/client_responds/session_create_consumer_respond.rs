use crate::commands::{SubscribeCommand, SuccessCommand};

use super::Respond;

pub struct SessionCreateConsumerRespond {}
impl Respond for SessionCreateConsumerRespond {
    type Request = SubscribeCommand;
    type Response = SuccessCommand;
    type Error = SessionCreateConsumerRespondError;
}

make_x_respond_error!(
    SessionCreateConsumer;
);
