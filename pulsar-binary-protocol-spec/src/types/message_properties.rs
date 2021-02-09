use std::fmt;

use seq_macro::seq;

use crate::protos::protobuf::pulsar_api::KeyValue;

#[derive(Clone)]
pub struct MessageProperties {
    #[cfg(feature = "with-hacking-commands")]
    pub inner: Vec<KeyValue>,
    #[cfg(not(feature = "with-hacking-commands"))]
    pub(crate) inner: Vec<KeyValue>,
}
impl fmt::Debug for MessageProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_vec())
    }
}

impl MessageProperties {
    pub fn to_vec(&self) -> Vec<(&str, &str)> {
        self.inner
            .iter()
            .map(|kv| (kv.get_key(), kv.get_value()))
            .collect::<Vec<_>>()
    }
}

seq!(N in 0..=10 {
    #(
        impl From<&[(&str, &str); N]> for MessageProperties {
            fn from(v: &[(&str, &str); N]) -> Self {
                let inner = v
                    .iter()
                    .map(|(k, v)| {
                        let mut kv = KeyValue::new();
                        kv.set_key(k.to_owned().into());
                        kv.set_value(v.to_owned().into());
                        kv
                    })
                    .collect::<Vec<_>>();

                Self { inner }
            }
        }

        impl From<Option<&[(&str, &str); N]>> for MessageProperties {
            fn from(v: Option<&[(&str, &str); N]>) -> Self {
                if let Some(v) = v {
                    v.into()
                } else {
                    Self {
                        inner: Default::default(),
                    }
                }
            }
        }
    )*
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(MessageProperties::from(&[("a", "1")]).inner.len(), 1);
        assert_eq!(MessageProperties::from(Some(&[("a", "1")])).inner.len(), 1);
    }
}
