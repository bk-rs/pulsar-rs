use super::protobuf::pulsar_api::KeyValue;

pub(crate) fn convert_tuple_slice_to_key_value_vector(slice: &[(&str, &str)]) -> Vec<KeyValue> {
    slice
        .iter()
        .map(|(k, v)| {
            let mut kv = KeyValue::new();
            kv.set_key(k.to_owned().into());
            kv.set_value(v.to_owned().into());
            kv
        })
        .collect::<Vec<_>>()
}
