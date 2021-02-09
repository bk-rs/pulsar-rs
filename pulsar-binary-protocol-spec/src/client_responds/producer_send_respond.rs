use crate::commands::{SendCommand, SendReceiptCommand};

use super::Respond;

pub struct ProducerSendRespond {}
impl Respond for ProducerSendRespond {
    type Request = SendCommand;
    type Response = SendReceiptCommand;
    type Error = ProducerSendRespondError;
}

make_x_respond_error!(
    ProducerSend;
);
