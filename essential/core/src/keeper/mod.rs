pub mod interaction;
pub mod subscription;

use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next, OnEvent};
use crb::core::{Slot, UniqueId};
use crb::superagent::{Entry, SubscribeExt, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use ice_nine_std::config_loader::{ConfigLoader, ConfigUpdates, NewConfig};
use interaction::GetConfig;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use subscription::{ConfigSegmentUpdates, Subscriber};
use toml::{Table, Value};

pub trait Config: DeserializeOwned + Serialize + Send + 'static {
    // TODO: Add scope
    // TODO: Add methods to get a full path for logging
    const NAMESPACE: &str;

    fn template() -> Self;
}

#[derive(Deref, DerefMut, From, Clone)]
pub struct KeeperLink {
    address: Address<Keeper>,
}

#[derive(From)]
pub struct MergedConfig {
    value: Value,
}

impl MergedConfig {
    fn new() -> Self {
        Self {
            value: Value::Table(Table::new()),
        }
    }
}

pub struct Keeper {
    config: MergedConfig,
    updater: Slot<Entry<ConfigUpdates>>,
    subscribers: HashMap<UniqueId<ConfigSegmentUpdates>, Subscriber>,
    loader: Slot<Address<ConfigLoader>>,
}

impl Keeper {
    pub fn new() -> Self {
        Self {
            config: MergedConfig::new(),
            updater: Slot::empty(),
            subscribers: HashMap::new(),
            loader: Slot::empty(),
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
        self.loader.fill(addr)?;

        // No subscribers here, not necessary to distribute the config
        self.config = MergedConfig::from(state_entry.state);
        self.updater.fill(state_entry.entry)?;
        Ok(Next::events())
    }
}

impl MergedConfig {
    fn get_config_segment(&self, seg: &GetConfig) -> Value {
        self.get_config_segment_opt(seg)
            .unwrap_or_else(|| seg.template.clone())
    }

    fn get_config_segment_opt(&self, seg: &GetConfig) -> Option<Value> {
        self.value
            .get("particle")?
            .get(&seg.namespace)?
            .get("config")
            .cloned()
    }
}

#[async_trait]
impl OnEvent<NewConfig> for Keeper {
    async fn handle(&mut self, config: NewConfig, _ctx: &mut Context<Self>) -> Result<()> {
        self.config = MergedConfig::from(config.0);
        self.distribute();
        Ok(())
    }
}
