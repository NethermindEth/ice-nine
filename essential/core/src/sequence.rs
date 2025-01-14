use crb::agent::{Agent, AgentSession};

/// `Sequence` is a small reasoning agent designed to bridge the model
/// with instruments until it gathers the complete context needed to generate a response.
pub struct Sequence {}

impl Agent for Sequence {
    type Context = AgentSession<Self>;
    type Output = ();
}
