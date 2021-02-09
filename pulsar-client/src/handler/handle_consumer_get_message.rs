use log::error;
use pulsar_binary_protocol_spec::{
    client_channel::FC_Sender,
    client_channel_messages::handler_reply_consumer_channel_message::HandlerReplyConsumerGetMessageChannelMessage,
    client_handler::PendingMessages, types::ConsumerId,
};

pub(super) fn handle_consumer_get_message(
    consumer_id: ConsumerId,
    sender: FC_Sender<HandlerReplyConsumerGetMessageChannelMessage>,
    pending_messages: &mut PendingMessages,
) {
    if let Some(pending_message_value) = pending_messages.get_mut(&consumer_id) {
        if !pending_message_value.is_empty() {
            let message_command = pending_message_value.remove(0);

            match sender.send(Some(message_command)) {
                Ok(_) => {}
                Err(message_command) => {
                    pending_message_value.push(message_command.unwrap());
                }
            }
        } else if sender.send(None).is_ok() {
        }
    } else {
        error!("not init consumer_id {:?}", consumer_id);
        unreachable!();
    }
}
