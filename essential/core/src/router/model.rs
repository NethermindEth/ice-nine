use super::types::{ChatRequest, ChatResponse, ToolingChatRequest, ToolingChatResponse};
use super::{ReasoningRouter, RouterLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, AddressExt, Equip, OnEvent};
use crb::superagent::{OnRequest, ResponseFetcher};
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
    fn chat(&self, request: ToolingChatRequest) -> ResponseFetcher<ToolingChatResponse>;
}

impl<M: Model> ModelAddress for Address<M> {
    fn chat(&self, request: ToolingChatRequest) -> ResponseFetcher<ToolingChatResponse> {
        self.interact(request)
    }
}

impl RouterLink {
    // TODO: Return model detacher (calls remove_model)
    pub fn add_model<M>(&mut self, addr: Address<M>) -> Result<()>
    where
        M: Model,
    {
        let msg = AddModel { link: addr.equip() };
        // TODO: Use interaction instead
        self.address.event(msg)?;
        Ok(())
    }

    pub fn chat(&self, request: ChatRequest) -> ResponseFetcher<ChatResponse> {
        self.interact(request)
    }
}

pub struct AddModel {
    link: ModelLink,
}

#[async_trait]
impl OnEvent<AddModel> for ReasoningRouter {
    async fn handle(&mut self, msg: AddModel, _ctx: &mut Self::Context) -> Result<()> {
        self.models.push(msg.link);
        Ok(())
    }
}
