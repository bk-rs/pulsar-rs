make_x_id_builder_and_x_id!(RequestId);

impl RequestId {
    pub fn is_require_set(&self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let builder = RequestIdBuilder::new(None);
        assert_eq!(u64::from(builder.next()), 1);
        assert_eq!(u64::from(builder.next()), 2);

        let builder = RequestIdBuilder::new(11);
        assert_eq!(u64::from(builder.next()), 11);
        assert_eq!(u64::from(builder.next()), 12);
    }
}
