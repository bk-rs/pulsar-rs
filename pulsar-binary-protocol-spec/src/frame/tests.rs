/*
cargo test --package pulsar-binary-protocol-spec --lib --features with-compression-zlib,with-compression-lz4 -- frame --nocapture
*/

use std::{convert::TryFrom, error};

use crate::{
    command::{CommandWithParsed, PayloadCommandPayloadWithParsed},
    commands::SendCommand,
    frame::{FrameParseOutput, FrameParser, FrameRenderer},
    protos::protobuf::pulsar_api::BaseCommand_Type as Type,
    types::{CompressionType, MessageProperties, ProducerId, ProducerName, SequenceId},
};

fn send_command_with_single(
    compression: Option<CompressionType>,
) -> Result<(), Box<dyn error::Error>> {
    let mut send_command = SendCommand::single(
        SequenceId::new(1),
        MessageProperties::from(&[("a", "1")]),
        b"foo",
        compression.to_owned(),
    );
    send_command.set_producer_id(ProducerId::new(1));
    send_command.set_producer_name(ProducerName::new("standalone-0-0"));

    let mut buf = Vec::new();
    FrameRenderer::new().render(&send_command, &mut buf)?;

    match FrameParser::new().parse(&buf[..])? {
        FrameParseOutput::Completed(n, c) => {
            assert_eq!(n, buf.len());
            match c {
                CommandWithParsed::Simple(c) => {
                    assert!(false, c)
                }
                CommandWithParsed::Payload(c) => {
                    let base_command = c.message;
                    let message_metadata = c.metadata;

                    assert_eq!(base_command.get_field_type(), Type::SEND);

                    let command_send = base_command.send.unwrap();
                    assert_eq!(command_send.get_sequence_id(), 1);
                    assert_eq!(command_send.get_producer_id(), 1);

                    assert_eq!(message_metadata.get_producer_name(), "standalone-0-0");
                    assert_eq!(message_metadata.get_num_messages_in_batch(), 1);
                    assert_eq!(
                        CompressionType::try_from(message_metadata.get_compression()).unwrap(),
                        compression.unwrap_or_else(|| CompressionType::NONE)
                    );
                    assert_eq!(message_metadata.properties.len(), 1);
                    assert_eq!(message_metadata.properties.first().unwrap().get_key(), "a");
                    assert_eq!(
                        message_metadata.properties.first().unwrap().get_value(),
                        "1"
                    );

                    match c.payload {
                        PayloadCommandPayloadWithParsed::Single(Ok(bytes)) => {
                            assert_eq!(bytes, b"foo")
                        }
                        PayloadCommandPayloadWithParsed::Single(Err(err)) => {
                            eprintln!("{:?}", err);
                            assert!(false, err)
                        }
                        _ => assert!(false),
                    }

                    assert_eq!(c.is_checksum_match, Some(true));
                }
            }
        }
        FrameParseOutput::Partial(n) => assert!(false, n),
    }

    Ok(())
}

#[test]
fn send_command_with_single_with_none() {
    assert!(send_command_with_single(None).is_ok())
}

#[test]
fn send_command_with_single_with_some_none() {
    assert!(send_command_with_single(Some(CompressionType::NONE)).is_ok())
}

#[cfg(feature = "with-compression-zlib")]
#[test]
fn send_command_with_single_with_some_zlib() {
    assert!(send_command_with_single(Some(CompressionType::ZLIB)).is_ok())
}

#[cfg(feature = "with-compression-lz4")]
#[test]
fn send_command_with_single_with_some_lz4() {
    assert!(send_command_with_single(Some(CompressionType::LZ4)).is_ok())
}

fn send_command_with_batch(
    compression: Option<CompressionType>,
) -> Result<(), Box<dyn error::Error>> {
    let mut send_command = SendCommand::batch(
        SequenceId::new(1),
        vec![(Some(&[("a", "1")]), b"foo"), (Some(&[("b", "2")]), b"bar")],
        compression.to_owned(),
    );
    send_command.set_producer_id(ProducerId::new(1));
    send_command.set_producer_name(ProducerName::new("standalone-0-0"));

    let mut buf = Vec::new();
    FrameRenderer::new()
        .render(&send_command, &mut buf)
        .unwrap();

    match FrameParser::new().parse(&buf[..]).unwrap() {
        FrameParseOutput::Completed(n, c) => {
            assert_eq!(n, buf.len());
            match c {
                CommandWithParsed::Simple(c) => {
                    assert!(false, c)
                }
                CommandWithParsed::Payload(c) => {
                    let base_command = c.message;
                    let message_metadata = c.metadata;

                    assert_eq!(base_command.get_field_type(), Type::SEND);

                    let command_send = base_command.send.unwrap();
                    assert_eq!(command_send.get_sequence_id(), 1);
                    assert_eq!(command_send.get_producer_id(), 1);

                    assert_eq!(message_metadata.get_producer_name(), "standalone-0-0");
                    assert_eq!(message_metadata.get_num_messages_in_batch(), 2);
                    assert_eq!(
                        CompressionType::try_from(message_metadata.get_compression()).unwrap(),
                        compression.unwrap_or_else(|| CompressionType::NONE)
                    );

                    match c.payload {
                        PayloadCommandPayloadWithParsed::Batch(Ok(msgs)) => {
                            assert_eq!(msgs.len(), 2);

                            let (single_message_metadata, bytes) = msgs[0].to_owned();
                            assert_eq!(single_message_metadata.properties.len(), 1);
                            assert_eq!(
                                single_message_metadata
                                    .properties
                                    .first()
                                    .unwrap()
                                    .get_key(),
                                "a"
                            );
                            assert_eq!(
                                single_message_metadata
                                    .properties
                                    .first()
                                    .unwrap()
                                    .get_value(),
                                "1"
                            );
                            assert_eq!(bytes, b"foo");

                            let (single_message_metadata, bytes) = msgs[1].to_owned();
                            assert_eq!(single_message_metadata.properties.len(), 1);
                            assert_eq!(
                                single_message_metadata
                                    .properties
                                    .first()
                                    .unwrap()
                                    .get_key(),
                                "b"
                            );
                            assert_eq!(
                                single_message_metadata
                                    .properties
                                    .first()
                                    .unwrap()
                                    .get_value(),
                                "2"
                            );
                            assert_eq!(bytes, b"bar")
                        }
                        PayloadCommandPayloadWithParsed::Batch(Err(err)) => {
                            eprintln!("{:?}", err);
                            assert!(false, err)
                        }
                        _ => assert!(false),
                    }

                    assert_eq!(c.is_checksum_match, Some(true));
                }
            }
        }
        FrameParseOutput::Partial(n) => assert!(false, n),
    }

    Ok(())
}

#[test]
fn send_command_with_batch_with_none() {
    assert!(send_command_with_batch(None).is_ok())
}

#[test]
fn send_command_with_batch_with_some_none() {
    assert!(send_command_with_batch(Some(CompressionType::NONE)).is_ok())
}

#[cfg(feature = "with-compression-zlib")]
#[test]
fn send_command_with_batch_with_some_zlib() {
    assert!(send_command_with_batch(Some(CompressionType::ZLIB)).is_ok())
}

#[cfg(feature = "with-compression-lz4")]
#[test]
fn send_command_with_batch_with_some_lz4() {
    assert!(send_command_with_batch(Some(CompressionType::LZ4)).is_ok())
}
