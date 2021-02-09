use std::collections::BTreeMap;

use crate::{
    client_channel::FC_Sender,
    client_channel_messages::handler_reply_producer_channel_message::HandlerReplyProducerSendChannelMessage,
    types::SequenceId,
};

pub type PendingSequences = BTreeMap<SequenceId, PendingSequenceValue>;

pub type PendingSequenceValue = FC_Sender<HandlerReplyProducerSendChannelMessage>;
