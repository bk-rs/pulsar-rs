use log::error;
use pulsar_binary_protocol_spec::{
    async_channel::bounded,
    client_channel::{FC_Sender, HandlerChannelStorage},
    client_channel_messages::{
        handler_reply_session_channel_message::HandlerReplySessionCreateConsumerChannelMessage,
        ConsumerSendHandlerChannelMessage,
    },
    client_responds::{Respond, SessionCreateConsumerRespond},
};

use super::HandleError;

pub(super) fn handle_session_create_consumer(
    consumer_command: <SessionCreateConsumerRespond as Respond>::Request,
    sender: FC_Sender<HandlerReplySessionCreateConsumerChannelMessage>,
    res: Result<
        <SessionCreateConsumerRespond as Respond>::Response,
        <SessionCreateConsumerRespond as Respond>::Error,
    >,
    channel_storage: &mut HandlerChannelStorage,
) -> Result<(), HandleError> {
    match res {
        Ok(c) => {
            let consumer_id = consumer_command.get_consumer_id();

            let (s, r) = bounded::<ConsumerSendHandlerChannelMessage>(10);
            channel_storage.add_consumer(consumer_id, r);
            match sender.send(Ok((consumer_command, c, s))) {
                Ok(_) => {}
                Err(_) => {
                    error!("channel closed");
                }
            }
        }
        Err(err) => match sender.send(Err(err)) {
            Ok(_) => {}
            Err(_) => {
                error!("channel closed");
            }
        },
    }

    Ok(())
}
