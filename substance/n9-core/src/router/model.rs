use super::types::{ToolingChatRequest, ToolingChatResponse};
use super::{ReasoningRouter, RouterLink};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Context, Equip, OnEvent};
use crb::superagent::{Fetcher, InteractExt, OnRequest, Request};
use derive_more::{Deref, DerefMut};
use std::sync::Arc;

pub trait Model: OnRequest<ToolingChatRequest> {}

#[derive(Deref, DerefMut, Clone)]
pub struct ModelLink {
    address: Arc<dyn ModelAddress>,
}

impl<M: Model> From<Address<M>> for ModelLink {
    fn from(addr: Address<M>) -> Self {
        Self {
            address: Arc::new(addr),
        }
    }
}

pub trait ModelAddress: Sync + Send {
    fn chat(&self, request: ToolingChatRequest) -> Fetcher<ToolingChatResponse>;
}

impl<M: Model> ModelAddress for Address<M> {
    fn chat(&self, request: ToolingChatRequest) -> Fetcher<ToolingChatResponse> {
        self.interact(request)
    }
}

impl RouterLink {
    // TODO: Return model detacher (calls remove_model)
    // Use subscriptions management to control model existence
    pub fn add_model<M>(&mut self, addr: Address<M>) -> Result<()>
    where
        M: Model,
    {
        let msg = AddModel { link: addr.equip() };
        // TODO: Use interaction instead
        self.address.event(msg)?;
        Ok(())
    }

    pub async fn get_model(&mut self) -> Result<ModelLink> {
        self.interact(GetModel).await.map_err(Error::from)
    }
}

pub struct AddModel {
    link: ModelLink,
}

#[async_trait]
impl OnEvent<AddModel> for ReasoningRouter {
    async fn handle(&mut self, msg: AddModel, _ctx: &mut Context<Self>) -> Result<()> {
        self.models.push(msg.link);
        Ok(())
    }
}

struct GetModel;

impl Request for GetModel {
    type Response = ModelLink;
}

#[async_trait]
impl OnRequest<GetModel> for ReasoningRouter {
    async fn on_request(&mut self, _: GetModel, ctx: &mut Context<Self>) -> Result<ModelLink> {
        self.models
            .first()
            .cloned()
            .ok_or_else(|| anyhow!("Models are not installed"))
    }
}
