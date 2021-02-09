use log::error;
use pulsar_binary_protocol_spec::{
    client_channel::FC_Sender,
    client_channel_messages::handler_reply_consumer_channel_message::HandlerReplyConsumerAckChannelMessage,
    client_responds::{ConsumerAckRespond, Respond},
};

use super::HandleError;

pub(super) fn handle_consumer_ack(
    sender: FC_Sender<HandlerReplyConsumerAckChannelMessage>,
    res: Result<<ConsumerAckRespond as Respond>::Response, <ConsumerAckRespond as Respond>::Error>,
) -> Result<(), HandleError> {
    // TODO

    match sender.send(res) {
        Ok(_) => {}
        Err(_) => {
            error!("channel closed");
        }
    }

    Ok(())
}
