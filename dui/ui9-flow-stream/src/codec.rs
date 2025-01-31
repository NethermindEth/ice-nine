pub trait Protocol: AsRef<str> + Clone + Send {
}

pub trait Codec: Send + 'static {
    type Protocol: Protocol;
}
