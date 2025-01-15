use super::{Config, Keeper, KeeperLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::Context;
use crb::superagent::{InteractExt, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};
use std::marker::PhantomData;
use toml::Value;

impl KeeperLink {
    pub async fn get_config<C>(&self) -> Result<C>
    where
        C: Config,
    {
        let request = GetConfig {
            namespace: C::NAMESPACE.to_string(),
        };
        let config = self.address.interact(request).await?.try_into()?;
        Ok(config)
    }
}

pub struct GetConfig {
    namespace: String,
    // TODO: Default in Arc
}

impl Request for GetConfig {
    type Response = Value;
}

#[async_trait]
impl OnRequest<GetConfig> for Keeper {
    async fn on_request(&mut self, msg: GetConfig, _: &mut Context<Self>) -> Result<Value> {
        let config = self.config.get_config(&msg.namespace);
        Ok(config)
    }
}
