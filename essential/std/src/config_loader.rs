use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, Duty, ManagedContext, Next, OnEvent, ReachableContext,
    ToAddress,
};
use crb::core::{Slot, SyncTag, UniqueId};
use crb::send::{Recipient, Sender};
use crb::superagent::{ManageSubscription, OnTimeout, Subscription, Timeout};
use derive_more::{Deref, DerefMut, From};
use notify::{
    recommended_watcher, Event, EventHandler, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use toml::Value;

const CONFIG_NAME: &str = "ice9.toml";

pub struct ConfigLoader {
    path: PathBuf,
    watcher: Slot<RecommendedWatcher>,
    debouncer: Slot<Timeout>,
    subscribers: HashSet<UniqueId<ConfigUpdates>>,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self {
            path: CONFIG_NAME.into(),
            watcher: Slot::empty(),
            debouncer: Slot::empty(),
            subscribers: HashSet::new(),
        }
    }
}

impl Agent for ConfigLoader {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }

    fn interrupt(&mut self, ctx: &mut Context<Self>) {
        self.watcher.take().ok();
        self.debouncer.take().ok();
        ctx.shutdown();
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for ConfigLoader {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let config_dir = dirs::config_dir();

        let forwarder = EventsForwarder::new(ctx.address().clone(), ());
        let mut watcher = recommended_watcher(forwarder)?;
        watcher.watch(&self.path, RecursiveMode::NonRecursive)?;
        self.watcher.fill(watcher)?;
        // TODO: Read the config here
        Ok(Next::events())
    }
}

#[derive(From)]
struct EventsForwarder<T> {
    tag: Arc<T>,
    address: Address<ConfigLoader>,
}

impl<T> EventsForwarder<T> {
    pub fn new(address: impl ToAddress<ConfigLoader>, tag: T) -> Self {
        Self {
            tag: Arc::new(tag),
            address: address.to_address(),
        }
    }
}

impl<T> EventHandler for EventsForwarder<T>
where
    T: SyncTag,
{
    fn handle_event(&mut self, result: WatchResult) {
        let event = WatchEvent {
            tag: self.tag.clone(),
            result,
        };
        self.address.event(event).ok();
    }
}

type WatchResult = Result<Event, notify::Error>;

struct WatchEvent<T> {
    tag: Arc<T>,
    result: WatchResult,
}

impl ConfigLoader {
    fn schedule_update(&mut self, ctx: &mut <Self as Agent>::Context) -> Result<()> {
        if self.debouncer.is_empty() {
            let address = ctx.address().clone();
            let duration = Duration::from_millis(250);
            let timeout = Timeout::new(address, duration, ());
            self.debouncer.fill(timeout)?;
        }
        Ok(())
    }

    async fn read_config(&mut self) -> Result<Value> {
        let content = fs::read_to_string(&self.path).await?;
        let value = toml::from_str(&content)?;
        Ok(value)
    }
}

#[async_trait]
impl<T> OnEvent<WatchEvent<T>> for ConfigLoader
where
    T: SyncTag,
{
    async fn handle(&mut self, msg: WatchEvent<T>, ctx: &mut Context<Self>) -> Result<()> {
        let event = msg.result?;
        println!("{:#?}", event.paths);
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                self.schedule_update(ctx)?;
            }
            _other => {
                // TODO: How to handle other methods? What if the config was removed?
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnTimeout for ConfigLoader {
    async fn on_timeout(&mut self, _: (), _ctx: &mut Context<Self>) -> Result<()> {
        self.debouncer.take()?;
        let value = self.read_config().await?;
        for subscriber in &self.subscribers {
            subscriber.send(NewConfig(value.clone())).ok();
        }
        Ok(())
    }
}

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
        let value = self.read_config().await?;
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
