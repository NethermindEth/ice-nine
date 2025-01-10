pub mod model;
pub mod tool;
pub mod types;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, Interaction, Next, OnRequest, OnResponse, Responder,
};
use crb::superagent::interaction::Output;
use derive_more::{Deref, DerefMut, From, Into};
use model::ModelLink;
use std::collections::HashMap;
use tool::{ToolId, ToolLink};
use typed_slab::TypedSlab;
use types::{ChatRequest, ToolingChatRequest, ToolingChatResponse};

#[derive(From, Into)]
pub struct ReqId(usize);

#[derive(Deref, DerefMut, From, Clone)]
pub struct RouterLink {
    address: Address<ReasoningRouter>,
}

pub struct ReasoningRouter {
    models: Vec<ModelLink>,
    tools: HashMap<ToolId, ToolLink>,
    requests: TypedSlab<ReqId, Responder<ChatRequest>>,
}

impl ReasoningRouter {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            tools: HashMap::new(),
            requests: TypedSlab::default(),
        }
    }
}

impl Agent for ReasoningRouter {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

#[async_trait]
impl OnRequest<ChatRequest> for ReasoningRouter {
    async fn handle(
        &mut self,
        lookup: Interaction<ChatRequest>,
        ctx: &mut Self::Context,
    ) -> Result<()> {
        // TODO: Picking model strategy
        let model = self
            .models
            .first()
            .ok_or_else(|| anyhow!("Models are not installed"))?;

        let address = ctx.address().clone();
        let req_id = self.requests.insert(lookup.responder);
        let request = lookup.request.with_tools();
        model.chat(request).forward_to(address, req_id);
        Ok(())
    }
}

#[async_trait]
impl OnResponse<ToolingChatRequest, ReqId> for ReasoningRouter {
    async fn on_response(
        &mut self,
        resp: Output<ToolingChatResponse>,
        req_id: ReqId,
        _ctx: &mut Self::Context,
    ) -> Result<()> {
        let responder = self
            .requests
            .remove(req_id)
            .ok_or_else(|| anyhow!("No responder"))?;
        let response = resp?.without_tools();
        responder.send(response)?;
        Ok(())
    }
}
