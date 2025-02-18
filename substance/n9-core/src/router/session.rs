use super::{ChatRequest, RouterLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Next};
use crb::superagent::{Interaction, OnRequest};

pub struct ReasoningSession {
    router: RouterLink,
}

impl Agent for ReasoningSession {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

#[async_trait]
impl OnRequest<ChatRequest> for ReasoningSession {
    async fn handle(
        &mut self,
        lookup: Interaction<ChatRequest>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        // ctx.do_next(Next::do_async(SendingRequest));
        Ok(())
    }
}
