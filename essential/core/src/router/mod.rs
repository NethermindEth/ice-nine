pub mod model;

use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, Equip, Interaction, Next, OnEvent, OnRequest,
    OnResponse, Responder,
};
use crb::core::Slot;
use crb::superagent::interaction::Output;
use derive_more::{Deref, DerefMut, From, Into};
use model::{ChatRequest, ChatResponse, Model, ModelLink};
use typed_slab::TypedSlab;

#[derive(From, Into)]
pub struct ReqId(usize);

#[derive(Deref, DerefMut, From, Clone)]
pub struct RouterLink {
    address: Address<Router>,
}

impl RouterLink {
    pub fn add_model<M>(&mut self, addr: Address<M>) -> Result<()>
    where
        M: Model,
    {
        let msg = AddModel { link: addr.equip() };
        self.address.event(msg)?;
        Ok(())
    }
}

pub struct Router {
    model: Slot<ModelLink>,
    requests: TypedSlab<ReqId, Responder<ChatRequest>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            model: Slot::empty(),
            requests: TypedSlab::default(),
        }
    }
}

impl Agent for Router {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

pub struct AddModel {
    link: ModelLink,
}

#[async_trait]
impl OnEvent<AddModel> for Router {
    type Error = Error;

    async fn handle(&mut self, msg: AddModel, _ctx: &mut Self::Context) -> Result<()> {
        self.model.fill(msg.link)?;
        Ok(())
    }
}

#[async_trait]
impl OnRequest<ChatRequest> for Router {
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
impl OnResponse<ChatRequest, ReqId> for Router {
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
