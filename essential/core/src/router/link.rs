use super::model::{Model, ModelLink};
use super::ReasoningRouter;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Equip, OnEvent};
use derive_more::{Deref, DerefMut, From};

#[derive(Deref, DerefMut, From, Clone)]
pub struct RouterLink {
    address: Address<ReasoningRouter>,
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

    pub fn model(&self) -> ModelLink {
        self.address.clone().equip()
    }
}

pub struct AddModel {
    link: ModelLink,
}

#[async_trait]
impl OnEvent<AddModel> for ReasoningRouter {
    type Error = Error;

    async fn handle(&mut self, msg: AddModel, _ctx: &mut Self::Context) -> Result<()> {
        self.model.fill(msg.link)?;
        Ok(())
    }
}
