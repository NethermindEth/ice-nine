use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, Duty, ManagedContext, Next, OnEvent};
use crb::core::Slot;
use crb::send::{Recipient, Sender};
use crb::superagent::{OnTimeout, Timeout};
use derive_more::From;
use notify::{
    recommended_watcher, Event, EventHandler, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use toml::Value;

const DEFAULT_PATH: &str = "ice9.toml";

pub struct ConfigLoader {
    path: PathBuf,
    watcher: Slot<RecommendedWatcher>,
    debouncer: Slot<Timeout>,
    recipient: Recipient<Value>,
}

impl ConfigLoader {
    pub fn new(recipient: Recipient<Value>) -> Self {
        Self {
            path: DEFAULT_PATH.into(),
            watcher: Slot::empty(),
            debouncer: Slot::empty(),
            recipient,
        }
    }
}

impl Agent for ConfigLoader {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Configure)
    }

    fn interrupt(&mut self, ctx: &mut Self::Context) {
        self.watcher.take().ok();
        self.debouncer.take().ok();
        ctx.shutdown();
    }
}

struct Configure;

#[async_trait]
impl Duty<Configure> for ConfigLoader {
    async fn handle(&mut self, _: Configure, ctx: &mut Self::Context) -> Result<Next<Self>> {
        let forwarder = EventsForwarder::from(ctx.address().clone());
        let mut watcher = recommended_watcher(forwarder)?;
        watcher.watch(&self.path, RecursiveMode::NonRecursive)?;
        self.watcher.fill(watcher)?;
        Ok(Next::events())
    }
}

#[derive(From)]
struct EventsForwarder {
    address: Address<ConfigLoader>,
}

impl EventHandler for EventsForwarder {
    fn handle_event(&mut self, event: EventResult) {
        self.address.event(event).ok();
    }
}

type EventResult = Result<Event, notify::Error>;

#[async_trait]
impl OnEvent<EventResult> for ConfigLoader {
    async fn handle(&mut self, result: EventResult, ctx: &mut Self::Context) -> Result<()> {
        let event = result?;
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                if self.debouncer.is_empty() {
                    let address = ctx.address().clone();
                    let duration = Duration::from_millis(250);
                    let timeout = Timeout::new(address, duration, ());
                    self.debouncer.fill(timeout)?;
                }
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
    async fn on_timeout(&mut self, _: (), _ctx: &mut Self::Context) -> Result<()> {
        self.debouncer.take()?;
        let content = fs::read_to_string(&self.path).await?;
        let value = toml::from_str(&content)?;
        self.recipient.send(value)?;
        Ok(())
    }
}
