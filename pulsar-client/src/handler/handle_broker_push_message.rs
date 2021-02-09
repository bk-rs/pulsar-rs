use log::error;
use pulsar_binary_protocol_spec::{client_handler::PendingMessages, MessageCommand};

pub(super) fn handle_broker_push_message(
    message_command: MessageCommand,
    pending_messages: &mut PendingMessages,
) {
    let consumer_id = message_command.get_consumer_id();

    if let Some(pending_message_value) = pending_messages.get_mut(&consumer_id) {
        pending_message_value.push(message_command);
    } else {
        error!("not init consumer_id {:?}", consumer_id);
        unreachable!();
    }
}
