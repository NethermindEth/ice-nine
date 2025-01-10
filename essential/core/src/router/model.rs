use super::types::{ChatRequest, ToolingChatRequest};
use super::{ReasoningRouter, RouterLink};
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Equip, OnEvent};
use crb::superagent::{Fetcher, OnRequest};
use derive_more::{Deref, DerefMut, From};
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
    fn chat(&self, request: ToolingChatRequest) -> Fetcher<ToolingChatRequest>;
}

impl<M: Model> ModelAddress for Address<M> {
    fn chat(&self, request: ToolingChatRequest) -> Fetcher<ToolingChatRequest> {
        self.interact(request)
    }
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

    pub fn chat(&self, request: ChatRequest) -> Fetcher<ChatRequest> {
        self.interact(request)
    }
}

pub struct AddModel {
    link: ModelLink,
}

#[async_trait]
impl OnEvent<AddModel> for ReasoningRouter {
    type Error = Error;

    async fn handle(&mut self, msg: AddModel, _ctx: &mut Self::Context) -> Result<()> {
        self.models.push(msg.link);
        Ok(())
    }
}
