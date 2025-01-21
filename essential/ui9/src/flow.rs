use serde::Serialize;

/// stdin flow
pub trait ControlFlow {}

/// stdout flow
pub trait EventFlow: Serialize {
    fn class() -> &'static str;
}

/// stderr flow
pub trait LogFlow {}
