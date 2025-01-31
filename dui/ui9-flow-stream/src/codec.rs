use std::fmt::Debug;

// TODO: Remove `Debug`
pub trait Protocol: Debug + AsRef<str> + Clone + Send {
}

impl<T> Protocol for T
where T: Debug + AsRef<str> + Clone + Send {}

pub trait Codec: Send + 'static {
    type Protocol: Protocol;
}
