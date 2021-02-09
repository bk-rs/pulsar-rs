use crate::protos::protobuf::pulsar_api::CommandAck_AckType as Protobuf_AckType;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AckType {
    Individual,
    Cumulative,
}

impl From<AckType> for Protobuf_AckType {
    fn from(at: AckType) -> Self {
        match at {
            AckType::Individual => Protobuf_AckType::Individual,
            AckType::Cumulative => Protobuf_AckType::Cumulative,
        }
    }
}
