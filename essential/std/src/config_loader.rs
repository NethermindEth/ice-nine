use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, Duty, ManagedContext, Next, OnEvent, ToAddress,
};
use crb::core::UniqueId;
use crb::send::{Recipient, Sender};
use crb::superagent::{ManageSubscription, Subscription, Timer, TimerHandle};
use derive_more::{Deref, DerefMut, From};
use notify::{
    recommended_watcher, Event, EventHandler, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use toml::{Table, Value};

const CONFIG_NAME: &str = "ice9.toml";
const TEMPLATE_NAME: &str = "ice9.example.toml";

pub struct ConfigLayer {
    path: Arc<PathBuf>,
    config: Value,
    _watcher: RecommendedWatcher,
}

impl ConfigLayer {
    async fn read_config(&mut self) -> Result<()> {
        log::info!("Reading the config layer: {}", self.path.display());
        let content = fs::read_to_string(self.path.as_ref()).await?;
        let config = toml::from_str(&content)?;
        self.config = config;
        Ok(())
    }
}

pub struct ChangedFiles {
    _debouncer: TimerHandle,
    files: HashSet<Arc<PathBuf>>,
}

impl ChangedFiles {
    fn new(debouncer: TimerHandle) -> Self {
        Self {
            _debouncer: debouncer,
            files: HashSet::new(),
        }
    }
}

pub struct ConfigLoader {
    layers: Vec<ConfigLayer>,
    changed_files: Option<ChangedFiles>,
    subscribers: HashSet<UniqueId<ConfigUpdates>>,
    merged_config: Value,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            changed_files: None,
            subscribers: HashSet::new(),
            merged_config: table(),
        }
    }
}

impl Agent for ConfigLoader {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }

    fn interrupt(&mut self, ctx: &mut Context<Self>) {
        self.changed_files.take();
        ctx.shutdown();
    }
}

impl ConfigLoader {
    async fn add_layer(&mut self, path: PathBuf, ctx: &mut Context<Self>) -> Result<()> {
        log::info!("Add a config layer: {}", path.display());
        let path = Arc::new(path);

        // Create a config file if doesn't exist
        if !path.exists() {
            log::info!("Creating an empty config layer: {}", path.display());
            fs::write(path.as_ref(), "").await?;
        }

        // Setup a watcher for file
        let forwarder = EventsForwarder::new(ctx, path.clone());
        let mut watcher = recommended_watcher(forwarder)?;
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        // Read a config
        let mut layer = ConfigLayer {
            path,
            config: table(),
            _watcher: watcher,
        };
        layer.read_config().await?;

        self.layers.push(layer);
        Ok(())
    }

    async fn update_configs(&mut self, all: bool) -> Result<()> {
        let changed_files = self
            .changed_files
            .take()
            .map(|record| record.files)
            .unwrap_or_default();
        let mut new_merged_config = table();
        for layer in &mut self.layers {
            if all || changed_files.contains(&layer.path) {
                layer.read_config().await?;
            }
            merge_configs(&mut new_merged_config, &layer.config);
        }
        if self.merged_config != new_merged_config {
            let new_config = NewConfig(new_merged_config.clone());
            for subscriber in &self.subscribers {
                subscriber.send(new_config.clone()).ok();
            }
            self.merged_config = new_merged_config;
        }
        Ok(())
    }

    fn schedule_update(&mut self, path: Arc<PathBuf>, ctx: &mut Context<Self>) -> Result<()> {
        match self.changed_files.as_mut() {
            Some(changed_files) => {
                changed_files.files.insert(path);
            }
            None => {
                let address = ctx.address().clone();
                let duration = Duration::from_millis(250);
                let timeout = Timer::just_spawn(address, duration, ());
                let mut changed_files = ChangedFiles::new(timeout);
                changed_files.files.insert(path);
                self.changed_files = Some(changed_files);
            }
        }
        Ok(())
    }

    fn current_config(&self) -> Value {
        self.merged_config.clone()
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for ConfigLoader {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // Global config layer: ~/.config/ice9.toml
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Config dir is not provided."))?
            .join(".config")
            .join("ice9");
        fs::create_dir_all(&config_dir).await?;
        let global_config = config_dir.join(CONFIG_NAME);
        self.add_layer(global_config, ctx).await?;

        // Local config layer: $PWD/ice9.toml
        let local_config = CONFIG_NAME.into();
        self.add_layer(local_config, ctx).await?;

        self.update_configs(true).await?;

        Ok(Next::events())
    }
}

#[derive(From)]
struct EventsForwarder {
    tag: Arc<PathBuf>,
    address: Address<ConfigLoader>,
}

impl EventsForwarder {
    pub fn new(address: impl ToAddress<ConfigLoader>, tag: Arc<PathBuf>) -> Self {
        Self {
            tag,
            address: address.to_address(),
        }
    }
}

impl EventHandler for EventsForwarder {
    fn handle_event(&mut self, result: WatchResult) {
        let event = WatchEvent {
            tag: self.tag.clone(),
            result,
        };
        self.address.event(event).ok();
    }
}

type WatchResult = Result<Event, notify::Error>;

struct WatchEvent {
    tag: Arc<PathBuf>,
    result: WatchResult,
}

#[async_trait]
impl OnEvent<WatchEvent> for ConfigLoader {
    async fn handle(&mut self, msg: WatchEvent, ctx: &mut Context<Self>) -> Result<()> {
        let event = msg.result?;
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                self.schedule_update(msg.tag, ctx)?;
            }
            _other => {
                // TODO: How to handle other methods? What if the config was removed?
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<()> for ConfigLoader {
    async fn handle(&mut self, _: (), _ctx: &mut Context<Self>) -> Result<()> {
        self.update_configs(false).await
    }
}

#[derive(Clone)]
pub struct NewConfig(pub Value);

#[derive(Deref, DerefMut)]
pub struct ConfigUpdates {
    recipient: Recipient<NewConfig>,
}

impl ConfigUpdates {
    pub fn for_listener<A>(addr: impl ToAddress<A>) -> Self
    where
        A: OnEvent<NewConfig>,
    {
        Self {
            recipient: addr.to_address().recipient(),
        }
    }
}

impl Subscription for ConfigUpdates {
    type State = Value;
}

#[async_trait]
impl ManageSubscription<ConfigUpdates> for ConfigLoader {
    async fn subscribe(
        &mut self,
        sub_id: UniqueId<ConfigUpdates>,
        _ctx: &mut Context<Self>,
    ) -> Result<Value> {
        // Read on initialze and keep
        self.subscribers.insert(sub_id);
        let value = self.current_config();
        Ok(value)
    }

    async fn unsubscribe(
        &mut self,
        sub_id: UniqueId<ConfigUpdates>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        self.subscribers.remove(&sub_id);
        Ok(())
    }
}

pub struct StoreTemplate(pub Value);

#[async_trait]
impl OnEvent<StoreTemplate> for ConfigLoader {
    async fn handle(&mut self, msg: StoreTemplate, _ctx: &mut Context<Self>) -> Result<()> {
        let content = toml::to_string_pretty(&msg.0)?;
        fs::write(TEMPLATE_NAME, content).await?;
        Ok(())
    }
}

pub fn wrap_level(title: &str, value: Value) -> Value {
    let mut wrapper = Table::new();
    wrapper.insert(title.into(), value);
    Value::Table(wrapper)
}

pub fn table() -> Value {
    Value::Table(Table::new())
}

pub fn merge_configs(base: &mut Value, overlay: &Value) {
    if let (Value::Table(base_table), Value::Table(overlay_table)) = (base, overlay) {
        for (key, overlay_value) in overlay_table {
            match base_table.get_mut(key) {
                Some(base_value) => {
                    // If both values are tables, recursively merge them
                    if overlay_value.is_table() && base_value.is_table() {
                        merge_configs(base_value, overlay_value);
                    } else {
                        // Otherwise, overlay value overwrites base value
                        *base_value = overlay_value.clone();
                    }
                }
                None => {
                    // If key doesn't exist in base, add it
                    base_table.insert(key.clone(), overlay_value.clone());
                }
            }
        }
    }
}
