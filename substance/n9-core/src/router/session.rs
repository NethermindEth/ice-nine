use super::{ChatRequest, ChatResponse, ReasoningRouter, RouterLink};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, Equip, Next, StopAddress, ToAddress};
use crb::superagent::{Fetcher, InteractExt, Interaction, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};

impl RouterLink {
    pub async fn new_session(&self) -> Result<SessionLink> {
        self.interact(NewSession).await.map_err(Error::from)
    }
}

struct NewSession;

impl Request for NewSession {
    type Response = SessionLink;
}

#[async_trait]
impl OnRequest<NewSession> for ReasoningRouter {
    async fn on_request(&mut self, _: NewSession, ctx: &mut Context<Self>) -> Result<SessionLink> {
        let link = ctx.equip();
        let session = ReasoningSession::new(link);
        let addr = ctx.spawn_agent(session, ());
        Ok(addr.equip())
    }
}

#[derive(Deref, DerefMut)]
pub struct SessionLink {
    address: StopAddress<ReasoningSession>,
}

impl From<Address<ReasoningSession>> for SessionLink {
    fn from(address: Address<ReasoningSession>) -> Self {
        Self {
            address: address.to_stop_address(),
        }
    }
}

impl SessionLink {
    pub fn chat(&self, request: ChatRequest) -> Fetcher<ChatResponse> {
        self.interact(request)
    }
}

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
