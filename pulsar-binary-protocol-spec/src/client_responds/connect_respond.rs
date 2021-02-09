use crate::commands::{ConnectCommand, ConnectedCommand};

use super::Respond;

pub struct ConnectRespond {}
impl Respond for ConnectRespond {
    type Request = ConnectCommand;
    type Response = ConnectedCommand;
    type Error = ConnectRespondError;
}

make_x_respond_error!(
    Connect;
    AuthenticationError "https://github.com/apache/pulsar/blob/v2.7.0/pulsar-broker/src/main/java/org/apache/pulsar/broker/service/ServerCnx.java#L773"
);
