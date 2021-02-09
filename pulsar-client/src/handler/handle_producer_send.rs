use log::error;
use pulsar_binary_protocol_spec::{
    client_channel::FC_Sender,
    client_channel_messages::handler_reply_producer_channel_message::HandlerReplyProducerSendChannelMessage,
    client_responds::{ProducerSendRespond, Respond},
};

use super::HandleError;

pub(super) fn handle_producer_send(
    sender: FC_Sender<HandlerReplyProducerSendChannelMessage>,
    res: Result<
        <ProducerSendRespond as Respond>::Response,
        <ProducerSendRespond as Respond>::Error,
    >,
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
