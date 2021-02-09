macro_rules! make_x_id_builder_and_x_id {
    ($name:ident) => {
        use std::{
            cmp::max,
            sync::atomic::{AtomicU64, Ordering},
        };

        paste! {
            #[derive(Debug)]
            pub struct [<$name Builder>](AtomicU64);
            impl [<$name Builder>] {
                pub fn new(initial_value: impl Into<Option<u64>>) -> Self {
                    Self(AtomicU64::new(
                        max(1, initial_value.into().unwrap_or_default())
                    ))
                }
                pub fn next(&self) -> $name {
                    $name(self.0.fetch_add(1, Ordering::SeqCst))
                }
            }
            impl Default for [<$name Builder>] {
                fn default() -> Self {
                    Self::new(None)
                }
            }
        }

        #[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
        pub struct $name(u64);
        impl From<$name> for u64 {
            fn from(v: $name) -> u64 {
                v.0
            }
        }
        impl $name {
            pub(crate) fn new(val: u64) -> Self {
                Self(val)
            }
        }
    };
}
