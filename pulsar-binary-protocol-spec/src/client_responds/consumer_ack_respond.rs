use crate::commands::{AckCommand, AckResponseCommand};

use super::Respond;

pub struct ConsumerAckRespond {}
impl Respond for ConsumerAckRespond {
    type Request = AckCommand;
    type Response = AckResponseCommand;
    type Error = ConsumerAckRespondError;
}

make_x_respond_error!(
    ConsumerAck;
);
