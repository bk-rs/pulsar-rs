use log::error;
use pulsar_binary_protocol_spec::{
    async_channel::bounded,
    client_channel::{FC_Sender, HandlerChannelStorage},
    client_channel_messages::{
        handler_reply_session_channel_message::HandlerReplySessionCreateProducerChannelMessage,
        ProducerSendHandlerChannelMessage,
    },
    client_responds::{Respond, SessionCreateProducerRespond},
};

use super::HandleError;

pub(super) fn handle_session_create_producer(
    producer_command: <SessionCreateProducerRespond as Respond>::Request,
    sender: FC_Sender<HandlerReplySessionCreateProducerChannelMessage>,
    res: Result<
        <SessionCreateProducerRespond as Respond>::Response,
        <SessionCreateProducerRespond as Respond>::Error,
    >,
    channel_storage: &mut HandlerChannelStorage,
) -> Result<(), HandleError> {
    match res {
        Ok(c) => {
            let producer_id = producer_command.get_producer_id();

            let (s, r) = bounded::<ProducerSendHandlerChannelMessage>(10);
            channel_storage.add_producer(producer_id, c.get_producer_name(), r);
            match sender.send(Ok((producer_command, c, s))) {
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
