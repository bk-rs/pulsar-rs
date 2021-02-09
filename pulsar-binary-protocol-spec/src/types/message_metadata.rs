use chrono::{DateTime, NaiveDateTime, Utc};

use crate::protos::protobuf::pulsar_api::MessageMetadata as Protobuf_MessageMetadata;

use super::message_properties::MessageProperties;

pub struct MessageMetadata<'a> {
    #[cfg(feature = "with-hacking-commands")]
    pub inner: &'a Protobuf_MessageMetadata,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner: &'a Protobuf_MessageMetadata,
}
impl<'a> MessageMetadata<'a> {
    pub fn get_publish_time(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(self.inner.get_publish_time() as i64, 0),
            Utc,
        )
    }

    pub fn get_properties(&self) -> MessageProperties {
        MessageProperties {
            inner: self.inner.properties.to_owned().into_vec(),
        }
    }
}

impl<'a> From<&'a Protobuf_MessageMetadata> for MessageMetadata<'a> {
    fn from(mm: &'a Protobuf_MessageMetadata) -> Self {
        Self { inner: mm }
    }
}
