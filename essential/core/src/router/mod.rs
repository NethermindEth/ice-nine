pub mod link;
pub mod model;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{
    Agent, AgentSession, Context, Interaction, Next, OnRequest, OnResponse, Responder,
};
use crb::core::Slot;
use crb::superagent::interaction::Output;
use derive_more::{From, Into};
use model::{ChatRequest, ChatResponse, Model, ModelLink};
use typed_slab::TypedSlab;

#[derive(From, Into)]
pub struct ReqId(usize);

pub struct ReasoningRouter {
    model: Slot<ModelLink>,
    requests: TypedSlab<ReqId, Responder<ChatRequest>>,
}

impl ReasoningRouter {
    pub fn new() -> Self {
        Self {
            model: Slot::empty(),
            requests: TypedSlab::default(),
        }
    }
}

impl Model for ReasoningRouter {}

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
        let model = self.model.get_mut()?;
        let address = ctx.address().clone();
        let req_id = self.requests.insert(lookup.responder);
        model.chat(lookup.request).forward_to(address, req_id);
        Ok(())
    }
}

#[async_trait]
impl OnResponse<ChatRequest, ReqId> for ReasoningRouter {
    async fn on_response(
        &mut self,
        resp: Output<ChatResponse>,
        req_id: ReqId,
        _ctx: &mut Self::Context,
    ) -> Result<()> {
        let responder = self
            .requests
            .remove(req_id)
            .ok_or_else(|| anyhow!("No responder"))?;
        let response = resp?;
        responder.send(response)?;
        Ok(())
    }
}
