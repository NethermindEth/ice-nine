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
        let model = self.router.get_model().await?;
        let tools = self.router.get_tools().await?;
        let request = request.with_tools(tools);
        let response = model.chat(request).await?;
        let response = response.without_tools();
        Ok(response)
    }
}
