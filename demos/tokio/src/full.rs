/*
bin/pulsar standalone
bin/pulsar-admin topics create public/default/my-topic
bin/pulsar-client consume my-topic -s "first-subscription" -n 100 -t Shared

RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p pulsar-demo-tokio --bin full 127.0.0.1 6650
*/

use std::{error, str, time::Duration};

use chrono::{Duration as ChronoDuration, Utc};
use futures_util::future::try_join_all;
use log::{debug, error, info};
use tokio::{net::TcpStream, task::spawn, time::sleep};

use pulsar_client::{
    spec::{
        types::{
            AckValidationError, CompressionType, MessageProperties, ProtocolVersion, SubscribeType,
        },
        AckCommand, ConnectCommand, FlowCommand, MessageCommandPayload, ProducerCommand,
        RedeliverUnacknowledgedMessagesCommand, SendCommand, SubscribeCommand,
    },
    tokio_io::{client::AsyncClient, connection::AsyncConnection},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();

    let tcp = TcpStream::connect("127.0.0.1:6650").await?;
    let conn = AsyncConnection::new(tcp, None);

    let mut connect_command = ConnectCommand::new("Pulsar-Client-Rust-v0.1.0");
    connect_command.set_protocol_version(ProtocolVersion::V17);
    let (sess, mut handler) = AsyncClient::new(conn).raw_connect(connect_command).await?;
    debug!("{:?}", sess);

    spawn(async move {
        // TODO, remove loop

        loop {
            match handler.handle().await {
                Ok(_) => {}
                Err((err, _channel_storage)) => {
                    error!("{:?}", err);
                }
            }
        }
    });

    //
    let mut consumers = vec![];
    for _ in 0..2 {
        let subscribe_command = SubscribeCommand::new(
            "persistent://public/default/my-topic",
            "first-subscription",
            SubscribeType::Shared,
        );
        let consumer = sess.raw_create_consumer(subscribe_command).await?;
        debug!("{:?}", consumer);
        consumers.push(consumer);
    }

    let mut producers = vec![];
    for _ in 0..2 {
        let producer_command = ProducerCommand::new("persistent://public/default/my-topic");
        let producer = sess.raw_create_producer(producer_command).await?;
        debug!("{:?}", producer);
        producers.push(producer);
    }

    let consumer1 = consumers.remove(0);

    //
    spawn(async move {
        let consumer = consumer1;
        let flow_command = FlowCommand::new(100);
        consumer.raw_flow(flow_command).await.unwrap();

        while let Ok(message_command) = consumer.get_message().await {
            if let Some(message_command) = message_command {
                let message_id = message_command.get_message_id().unwrap();

                if message_command.get_is_checksum_mismatch() == Some(true) {
                    error!("{:?} {:?}", consumer.get_consumer_id(), message_id);

                    let ack_command =
                        AckCommand::individual(&[message_id], AckValidationError::ChecksumMismatch);
                    consumer.raw_ack(ack_command).await.unwrap();
                    continue;
                }

                match message_command.get_payload() {
                    MessageCommandPayload::Single(Ok(bytes)) => {
                        if fastrand::u8(1..10) < 4 {
                            info!(
                                "{:?} {:?} {:?} {:?}",
                                consumer.get_consumer_id(),
                                message_command.get_message_metadata().get_properties(),
                                str::from_utf8(bytes),
                                (message_id.get_ledger_id(), message_id.get_entry_id())
                            );

                            let ack_command =
                                AckCommand::individual(&[message_id.to_owned()], None);
                            consumer.raw_ack(ack_command).await.unwrap();
                        } else {
                            info!(
                                "{:?} redeliver {:?} {:?} {:?}",
                                consumer.get_consumer_id(),
                                message_command.get_message_metadata().get_properties(),
                                str::from_utf8(bytes),
                                (message_id.get_ledger_id(), message_id.get_entry_id())
                            );
                            let redeliver_unacknowledged_messages_command =
                                RedeliverUnacknowledgedMessagesCommand::new(&[message_id]);
                            consumer
                                .raw_redeliver_unacknowledged_messages(
                                    redeliver_unacknowledged_messages_command,
                                )
                                .await
                                .unwrap();
                        }
                    }
                    MessageCommandPayload::Single(Err(err)) => {
                        error!(
                            "{:?} {:?} {:?}",
                            consumer.get_consumer_id(),
                            message_id,
                            err
                        );

                        let ack_command =
                            AckCommand::individual(&[message_id], AckValidationError::from(err));
                        consumer.raw_ack(ack_command).await.unwrap();
                    }
                    MessageCommandPayload::Batch(Ok(msgs)) => {
                        info!(
                            "{:?} {:?} {:?}",
                            consumer.get_consumer_id(),
                            msgs.iter()
                                .map(|(single_message_metadata, bytes)| (
                                    single_message_metadata.get_properties(),
                                    str::from_utf8(bytes)
                                ))
                                .collect::<Vec<_>>(),
                            (message_id.get_ledger_id(), message_id.get_entry_id())
                        );

                        let ack_command = AckCommand::individual(&[message_id], None);
                        consumer.raw_ack(ack_command).await.unwrap();
                    }
                    MessageCommandPayload::Batch(Err(err)) => {
                        error!(
                            "{:?} {:?} {:?}",
                            consumer.get_consumer_id(),
                            message_id,
                            err
                        );

                        let ack_command =
                            AckCommand::individual(&[message_id], AckValidationError::from(err));
                        consumer.raw_ack(ack_command).await.unwrap();
                    }
                }
            } else {
                sleep(Duration::from_secs(1)).await;
            }
        }
    });

    //
    let producer = producers.first().unwrap();

    let mut futures = vec![];
    for i in 0..10 {
        let mut send_command = SendCommand::single(
            producer.next_sequence_id(),
            MessageProperties::from(&[("a", "1")]),
            format!("a1 {} {}", i, Utc::now()).as_bytes(),
            CompressionType::ZLIB,
        );

        if i % 2 == 0 {
            send_command.set_deliver_at_time(Utc::now() + ChronoDuration::seconds(5));
        }

        futures.push(producer.raw_send(send_command));
    }
    let rets = try_join_all(futures).await;
    debug!("{:?}", rets);

    for i in 0..2 {
        let send_command = SendCommand::batch(
            producer.next_sequence_id(),
            vec![
                (
                    Some(&[("b", "2")]),
                    format!("b2 {} {}", i, Utc::now()).as_bytes(),
                ),
                (
                    Some(&[("c", "3")]),
                    format!("c3 {} {}", i, Utc::now()).as_bytes(),
                ),
            ],
            CompressionType::ZLIB,
        );
        let send_receipt_command = producer.raw_send(send_command).await?;
        debug!("{:?}", send_receipt_command);
    }

    sleep(Duration::from_secs(30)).await;

    Ok(())
}
