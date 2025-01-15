pub mod interaction;
pub mod subscription;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{
    Entry, InteractExt, OnRequest, Request, Subscribe, SubscribeExt, Subscription, Supervisor,
    SupervisorSession,
};
use derive_more::{Deref, DerefMut, From};
use ice_nine_std::config_loader::{ConfigLoader, ConfigUpdates, NewConfig};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use toml::Value;

pub trait Config: DeserializeOwned + Send + 'static {
    // TODO: Add scope
    // TODO: Add methods to get a full path for logging
    const NAMESPACE: &str;
}

#[derive(Deref, DerefMut, From, Clone)]
pub struct KeeperLink {
    address: Address<Keeper>,
}

pub struct Keeper {
    config: Option<Value>,
    updater: Slot<Entry<ConfigUpdates>>,
}

impl Keeper {
    pub fn new() -> Self {
        Self {
            config: None,
            updater: Slot::empty(),
        }
    }
}

impl Supervisor for Keeper {
    type GroupBy = ();
}

impl Agent for Keeper {
    type Context = SupervisorSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for Keeper {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let loader = ConfigLoader::new();
        let (addr, _) = ctx.spawn_agent(loader, ());
        let sub = ConfigUpdates::for_listener(ctx);
        let state_entry = addr.subscribe(sub).await?;
        // No subscribers here, not necessary to distribute the config
        self.config = Some(state_entry.state);
        self.updater.fill(state_entry.entry)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<NewConfig> for Keeper {
    async fn handle(&mut self, config: NewConfig, ctx: &mut Context<Self>) -> Result<()> {
        self.config = Some(config.0);
        // TODO: Distribute updates
        Ok(())
    }
}
