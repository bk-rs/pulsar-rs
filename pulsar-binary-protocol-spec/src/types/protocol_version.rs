use seq_macro::seq;

use crate::protos::protobuf::pulsar_api::ProtocolVersion as Protobuf_ProtocolVersion;

seq!(N in 0..=17 {
    #[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
    pub enum ProtocolVersion {
        #(
            V#N,
        )*
    }
});

seq!(N in 0..=17 {
    impl From<ProtocolVersion> for Protobuf_ProtocolVersion {
        fn from(pv: ProtocolVersion) -> Self {
            match pv {
                #(
                    ProtocolVersion::V#N => Self::v#N,
                )*
            }
        }
    }
});

seq!(N in 0..=17 {
    impl From<Protobuf_ProtocolVersion> for ProtocolVersion {
        fn from(pv: Protobuf_ProtocolVersion) -> Self {
            match pv {
                #(
                    Protobuf_ProtocolVersion::v#N => Self::V#N,
                )*
            }
        }
    }
});
