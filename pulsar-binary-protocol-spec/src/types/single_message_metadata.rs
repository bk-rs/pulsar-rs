use crate::protos::protobuf::pulsar_api::SingleMessageMetadata as Protobuf_SingleMessageMetadata;

use super::message_properties::MessageProperties;

#[derive(Debug)]
pub struct SingleMessageMetadata<'a> {
    #[cfg(feature = "with-hacking-commands")]
    pub inner: &'a Protobuf_SingleMessageMetadata,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner: &'a Protobuf_SingleMessageMetadata,
}
impl<'a> SingleMessageMetadata<'a> {
    pub fn get_properties(&self) -> MessageProperties {
        MessageProperties {
            inner: self.inner.properties.to_owned().into_vec(),
        }
    }
}

impl<'a> From<&'a Protobuf_SingleMessageMetadata> for SingleMessageMetadata<'a> {
    fn from(smm: &'a Protobuf_SingleMessageMetadata) -> Self {
        Self { inner: smm }
    }
}
