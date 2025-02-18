use super::{ChatRequest, ChatResponse, RouterLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Next};
use crb::superagent::{Interaction, OnRequest};

pub struct ReasoningSession {
    router: RouterLink,
}

impl ReasoningSession {
    pub fn new(router: RouterLink) -> Self {
        Self { router }
    }
}

impl Agent for ReasoningSession {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

#[async_trait]
impl OnRequest<ChatRequest> for ReasoningSession {
    async fn on_request(
        &mut self,
        request: ChatRequest,
        ctx: &mut Context<Self>,
    ) -> Result<ChatResponse> {
        // ctx.do_next(Next::do_async(SendingRequest));
        Err(anyhow!("Not implemented"))
    }
}
