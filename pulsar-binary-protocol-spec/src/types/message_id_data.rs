use crate::protos::protobuf::pulsar_api::MessageIdData as Protobuf_MessageIdData;

#[derive(Debug, Clone)]
pub struct MessageIdData {
    #[cfg(feature = "with-hacking-commands")]
    pub inner: Protobuf_MessageIdData,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner: Protobuf_MessageIdData,
}
impl MessageIdData {
    pub fn get_ledger_id(&self) -> u64 {
        self.inner.get_ledgerId()
    }

    pub fn get_entry_id(&self) -> u64 {
        self.inner.get_entryId()
    }
}

impl From<&Protobuf_MessageIdData> for MessageIdData {
    fn from(mid: &Protobuf_MessageIdData) -> Self {
        Self {
            inner: mid.to_owned(),
        }
    }
}
