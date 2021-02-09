use std::io::ErrorKind as IoErrorKind;

use log::{error, trace};
use pulsar_binary_protocol_spec::{
    async_channel::TryRecvError,
    client_channel::{AC_Receiver, HandlerChannelStorage, HandlerChannelStorageItem},
    client_channel_messages::{
        consumer_send_handler_channel_message::ConsumerSendHandlerChannelMessageGroup,
        producer_send_handler_channel_message::ProducerSendHandlerChannelMessageGroup,
        SessionSendHandlerChannelMessage,
    },
    client_handler::{
        handle, HandlerHandleOutput, OnResponded, PendingMessages, PendingRequests,
        PendingSequences, ReadCommandError,
    },
    PongCommand,
};
use thiserror::Error;

use super::{connection::AsyncConnection, AsyncRead, AsyncWrite};

mod handle_broker_pong;
mod handle_broker_push_message;
mod handle_consumer_ack;
mod handle_consumer_get_message;
mod handle_producer_send;
mod handle_session_create_consumer;
mod handle_session_create_producer;

pub struct AsyncHandler<S> {
    connection: AsyncConnection<S>,
    channel_storage: HandlerChannelStorage,
    pending_requests: PendingRequests,
    pending_sequences: PendingSequences,
    pending_messages: PendingMessages,
}

impl<S> AsyncHandler<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub(crate) fn new(
        connection: AsyncConnection<S>,
        receiver: AC_Receiver<SessionSendHandlerChannelMessage>,
    ) -> Self {
        Self {
            connection,
            channel_storage: HandlerChannelStorage::new(receiver),
            pending_requests: PendingRequests::default(),
            pending_sequences: PendingSequences::default(),
            pending_messages: PendingMessages::default(),
        }
    }
}

#[derive(Error, Debug)]
pub enum HandleError {
    #[error("ConnectionRequireClose")]
    ConnectionRequireClose,
}

impl<S> AsyncHandler<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn handle(&mut self) -> Result<(), (HandleError, HandlerChannelStorage)> {
        //
        let mut channel_storage_del_session = false;
        let mut channel_storage_del_producer_ids = vec![];
        let mut channel_storage_del_consumer_ids = vec![];
        for item in self.channel_storage.items().into_iter() {
            match item {
                HandlerChannelStorageItem::Session(r) => match r.try_recv() {
                    Ok(msg) => {
                        let ((request_id, pending_request), command) = msg
                            .into_pending_request_and_command(
                                &self.connection.request_id_builder,
                                &self.connection.producer_id_builder,
                                &self.connection.consumer_id_builder,
                            );

                        match self.connection.write_command(command).await {
                            Ok(_) => {
                                self.pending_requests.insert(request_id, pending_request);
                            }
                            Err(err) => {
                                error!("{:?}", err);
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Closed) => {
                        channel_storage_del_session = true;
                    }
                },
                HandlerChannelStorageItem::Producer(producer_id, producer_name, r) => match r
                    .try_recv()
                {
                    Ok(msg) => {
                        let group = msg.into_group(producer_id, producer_name.to_owned());

                        match group {
                            ProducerSendHandlerChannelMessageGroup::PendingSequence(
                                sequence_id,
                                pending_sequence,
                                command,
                            ) => match self.connection.write_command(command).await {
                                Ok(_) => {
                                    self.pending_sequences.insert(sequence_id, pending_sequence);
                                }
                                Err(err) => {
                                    error!("{:?}", err);
                                }
                            },
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Closed) => {
                        channel_storage_del_producer_ids.push(producer_id);
                    }
                },

                HandlerChannelStorageItem::Consumer(consumer_id, r) => match r.try_recv() {
                    Ok(msg) => {
                        let group = msg.into_group(
                            consumer_id.to_owned(),
                            &self.connection.request_id_builder,
                        );

                        match group {
                            ConsumerSendHandlerChannelMessageGroup::Flow(command, s) => {
                                self.pending_messages
                                    .entry(consumer_id.to_owned())
                                    .or_insert_with(Default::default);

                                match self.connection.write_command(command).await {
                                    Ok(_) => match s.send(Ok(())) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            error!("channel closed");
                                        }
                                    },
                                    Err(err) => match s.send(Err(err.into())) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            error!("channel closed");
                                        }
                                    },
                                }
                            }
                            ConsumerSendHandlerChannelMessageGroup::GetMessage(s) => {
                                handle_consumer_get_message::handle_consumer_get_message(
                                    consumer_id,
                                    s,
                                    &mut self.pending_messages,
                                )
                            }
                            ConsumerSendHandlerChannelMessageGroup::PendingRequest(
                                request_id,
                                pending_request,
                                command,
                            ) => match self.connection.write_command(*command).await {
                                Ok(_) => {
                                    self.pending_requests.insert(request_id, pending_request);
                                }
                                Err(err) => {
                                    error!("{:?}", err);
                                }
                            },
                            ConsumerSendHandlerChannelMessageGroup::RedeliverUnacknowledgedMessages(command, s) => {
                                match self.connection.write_command(command).await {
                                    Ok(_) => match s.send(Ok(())) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            error!("channel closed");
                                        }
                                    },
                                    Err(err) => match s.send(Err(err.into())) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            error!("channel closed");
                                        }
                                    },
                                }
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Closed) => {
                        channel_storage_del_consumer_ids.push(consumer_id);
                    }
                },
            }
        }
        if channel_storage_del_session {
            self.channel_storage.del_session();
        }
        for producer_id in channel_storage_del_producer_ids {
            self.channel_storage.del_producer(producer_id);
        }
        for consumer_id in channel_storage_del_consumer_ids {
            self.channel_storage.del_consumer(consumer_id);
        }

        //

        match self.connection.try_read_commands(None).await {
            Ok(Some(commands)) => {
                for command in commands.iter() {
                    match handle(command, &mut self.pending_requests, &mut self.pending_sequences) {
                        Ok(output) => {
                            match output {
                                HandlerHandleOutput::BrokerPing(_) => {
                                    trace!("receive BrokerPing");

                                    let c= PongCommand::new();
                                    match self.connection.write_command(&c).await {
                                        Ok(_) => {},
                                        Err(err) => {
                                            error!("{:?}", err);
                                        }
                                    };
                                }
                                HandlerHandleOutput::OnPingResponded(c) => {
                                    trace!("receive BrokerPong");

                                    match handle_broker_pong::handle_broker_pong(c) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            error!("{:?}", err);
                                        }
                                    }
                                }
                                HandlerHandleOutput::OnConnectResponded(_) => {
                                    error!("unreachable");
                                }
                                HandlerHandleOutput::OnResponded(
                                    res,
                                ) => match *res {
                                    OnResponded::SessionCreateProducer(producer_command, s, res) => {
                                        match handle_session_create_producer::handle_session_create_producer(producer_command.to_owned(),s, res, &mut self.channel_storage) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                error!("{:?}", err);
                                            }
                                        }
                                    }
                                    OnResponded::SessionCreateConsumer(subscribe_command,s, res) => {
                                        match handle_session_create_consumer::handle_session_create_consumer(subscribe_command.to_owned(),s, res, &mut self.channel_storage) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                error!("{:?}", err);
                                            }
                                        }
                                    }
                                    OnResponded::ProducerSend(s,res) => {
                                        match handle_producer_send::handle_producer_send(s, res) {
                                            Ok(_) => {}
                                                Err(err) => {
                                                    error!("{:?}", err);
                                                }
                                        }
                                    }
                                    OnResponded::ConsumerAck(s,res) => {
                                        match handle_consumer_ack::handle_consumer_ack(s, res) {
                                            Ok(_) => {}
                                                Err(err) => {
                                                    error!("{:?}", err);
                                                }
                                        }
                                    }
                                },
                                HandlerHandleOutput::BrokerPushMessage(c) => {
                                     handle_broker_push_message::handle_broker_push_message(*c, &mut self.pending_messages)
                                }
                            }
                        }
                        Err(err) => {
                            error!("{:?}", err);
                        }
                    }
                }
            }
            Ok(None) => {}
            Err(err) => match err {
                ReadCommandError::ReadError(err) if err.kind() == IoErrorKind::TimedOut => {}
                _ => {
                    error!("{:?}", err);
                    unimplemented!()
                }
            },
        }

        Ok(())
    }
}
