use super::{Config, Keeper, KeeperLink};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::Context;
use crb::superagent::{InteractExt, OnRequest, Request};
use toml::Value;

impl KeeperLink {
    pub async fn get_config<C>(&self) -> Result<C>
    where
        C: Config,
    {
        let request = GetConfig::new::<C>()?;
        let config = self.address.interact(request).await?.try_into()?;
        Ok(config)
    }
}

pub struct GetConfig {
    pub namespace: String,
    pub template: Value,
}

impl GetConfig {
    pub fn new<C: Config>() -> Result<Self> {
        let namespace = C::NAMESPACE.to_string();
        let template = Value::try_from(C::template())?;
        Ok(Self {
            namespace,
            template,
        })
    }
}

impl Request for GetConfig {
    type Response = Value;
}

#[async_trait]
impl OnRequest<GetConfig> for Keeper {
    async fn on_request(&mut self, msg: GetConfig, _: &mut Context<Self>) -> Result<Value> {
        let config = self.config.get_config_segment(&msg);
        Ok(config)
    }
}
