pub mod model;
pub mod session;
pub mod tool;
pub mod types;

use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, Equip, Next};
use crb::superagent::{InteractExt, OnRequest, Request, Responder, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From, Into};
use model::ModelLink;
use session::{ReasoningSession, SessionLink};
use std::collections::HashMap;
use tool::{ToolId, ToolRecord};
use typed_slab::TypedSlab;
use types::{ChatRequest, ChatResponse};

#[derive(From, Into)]
pub struct ReqId(usize);

#[derive(Deref, DerefMut, From, Clone)]
pub struct RouterLink {
    address: Address<ReasoningRouter>,
}

pub struct ReasoningRouter {
    models: Vec<ModelLink>,
    tools: HashMap<ToolId, ToolRecord>,
    requests: TypedSlab<ReqId, Responder<ChatResponse>>,
}

impl ReasoningRouter {
    pub fn new() -> Self {
        Self {
            models: Vec::default(),
            tools: HashMap::default(),
            requests: TypedSlab::default(),
        }
    }
}

impl Supervisor for ReasoningRouter {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for ReasoningRouter {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

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
