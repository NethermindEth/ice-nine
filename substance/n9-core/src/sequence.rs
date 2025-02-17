use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};

/// `Sequence` is a small reasoning agent designed to bridge the model
/// with instruments until it gathers the complete context needed to generate a response.
pub struct Sequence {}

impl Agent for Sequence {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Sequence {
    async fn handle(&mut self, _: Initialize, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::done())
    }
}
