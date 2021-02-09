#[derive(Debug, Clone)]
pub struct ProducerName(String);
impl ProducerName {
    pub(crate) fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<ProducerName> for String {
    fn from(v: ProducerName) -> String {
        v.0
    }
}

/*
standalone-0-0
standalone-0-1

standalone-1-0
standalone-1-1

standalone-2-0
standalone-2-1
*/
