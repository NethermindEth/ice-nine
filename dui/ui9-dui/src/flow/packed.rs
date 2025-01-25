use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! packed {
    ($name:ident) => {
        #[derive(Clone, From, Into, Serialize, Deserialize, PartialEq, Eq)]
        pub struct $name(pub Vec<u8>);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("size", &self.0.len())
                    .finish()
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
    };
}

packed!(PackedState);
packed!(PackedEvent);
packed!(PackedAction);
