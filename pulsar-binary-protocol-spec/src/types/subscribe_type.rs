use crate::protos::protobuf::pulsar_api::CommandSubscribe_SubType as Protobuf_SubType;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SubscribeType {
    Exclusive,
    Shared,
    Failover,
    KeyShared,
}

impl From<SubscribeType> for Protobuf_SubType {
    fn from(st: SubscribeType) -> Self {
        match st {
            SubscribeType::Exclusive => Protobuf_SubType::Exclusive,
            SubscribeType::Shared => Protobuf_SubType::Shared,
            SubscribeType::Failover => Protobuf_SubType::Failover,
            SubscribeType::KeyShared => Protobuf_SubType::Key_Shared,
        }
    }
}
