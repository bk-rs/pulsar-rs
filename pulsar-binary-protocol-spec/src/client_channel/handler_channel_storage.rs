use std::collections::HashMap;

use crate::{
    client_channel_messages::{
        ConsumerSendHandlerChannelMessage, ProducerSendHandlerChannelMessage,
        SessionSendHandlerChannelMessage,
    },
    types::{ConsumerId, ProducerId, ProducerName},
};

use super::AC_Receiver;

pub type SessionReceiver = AC_Receiver<SessionSendHandlerChannelMessage>;
pub type ProducerReceiver = AC_Receiver<ProducerSendHandlerChannelMessage>;
pub type ConsumerReceiver = AC_Receiver<ConsumerSendHandlerChannelMessage>;

#[derive(Debug)]
pub struct HandlerChannelStorage(HashMap<HandlerChannelStorageKey, HandlerChannelStorageValue>);
impl HandlerChannelStorage {
    pub fn new(session_receiver: SessionReceiver) -> Self {
        let mut map = HashMap::new();
        map.insert(
            HandlerChannelStorageKey::Session,
            HandlerChannelStorageValue::Session(session_receiver),
        );
        Self(map)
    }

    pub fn del_session(&mut self) -> bool {
        self.0.remove(&HandlerChannelStorageKey::Session).is_some()
    }

    pub fn get_session(&self) -> Option<&SessionReceiver> {
        match self.0.get(&HandlerChannelStorageKey::Session) {
            Some(HandlerChannelStorageValue::Session(r)) => Some(r),
            _ => None,
        }
    }

    //
    pub fn add_producer(
        &mut self,
        producer_id: ProducerId,
        producer_name: ProducerName,
        receiver: ProducerReceiver,
    ) -> bool {
        self.0
            .insert(
                HandlerChannelStorageKey::Producer(producer_id),
                HandlerChannelStorageValue::Producer(producer_name, receiver),
            )
            .is_none()
    }

    pub fn del_producer(&mut self, producer_id: ProducerId) -> bool {
        self.0
            .remove(&HandlerChannelStorageKey::Producer(producer_id))
            .is_some()
    }

    pub fn get_producer(
        &self,
        producer_id: ProducerId,
    ) -> Option<(&ProducerName, &ProducerReceiver)> {
        match self.0.get(&HandlerChannelStorageKey::Producer(producer_id)) {
            Some(HandlerChannelStorageValue::Producer(producer_name, r)) => {
                Some((producer_name, r))
            }
            None => None,
            _ => unreachable!(),
        }
    }

    //
    pub fn add_consumer(&mut self, consumer_id: ConsumerId, receiver: ConsumerReceiver) -> bool {
        self.0
            .insert(
                HandlerChannelStorageKey::Consumer(consumer_id),
                HandlerChannelStorageValue::Consumer(receiver),
            )
            .is_none()
    }

    pub fn del_consumer(&mut self, consumer_id: ConsumerId) -> bool {
        self.0
            .remove(&HandlerChannelStorageKey::Consumer(consumer_id))
            .is_some()
    }

    pub fn get_consumer(&self, consumer_id: ConsumerId) -> Option<&ConsumerReceiver> {
        match self.0.get(&HandlerChannelStorageKey::Consumer(consumer_id)) {
            Some(HandlerChannelStorageValue::Consumer(r)) => Some(r),
            None => None,
            _ => unreachable!(),
        }
    }

    //
    pub fn items(&self) -> Vec<HandlerChannelStorageItem<'_>> {
        let mut items = vec![];

        for key in self.0.keys() {
            match key {
                HandlerChannelStorageKey::Session => {
                    if let Some(r) = self.get_session() {
                        items.push(HandlerChannelStorageItem::Session(r))
                    }
                }
                HandlerChannelStorageKey::Producer(producer_id) => {
                    if let Some((producer_name, r)) = self.get_producer(producer_id.to_owned()) {
                        items.push(HandlerChannelStorageItem::Producer(
                            producer_id.to_owned(),
                            producer_name,
                            r,
                        ))
                    } else {
                        unimplemented!()
                    }
                }
                HandlerChannelStorageKey::Consumer(consumer_id) => {
                    if let Some(r) = self.get_consumer(consumer_id.to_owned()) {
                        items.push(HandlerChannelStorageItem::Consumer(
                            consumer_id.to_owned(),
                            r,
                        ))
                    } else {
                        unimplemented!()
                    }
                }
            }
        }

        items
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum HandlerChannelStorageKey {
    Session,
    Producer(ProducerId),
    Consumer(ConsumerId),
}

#[derive(Debug)]
enum HandlerChannelStorageValue {
    Session(SessionReceiver),
    Producer(ProducerName, ProducerReceiver),
    Consumer(ConsumerReceiver),
}

#[derive(Debug)]
pub enum HandlerChannelStorageItem<'a> {
    Session(&'a SessionReceiver),
    Producer(ProducerId, &'a ProducerName, &'a ProducerReceiver),
    Consumer(ConsumerId, &'a ConsumerReceiver),
}
