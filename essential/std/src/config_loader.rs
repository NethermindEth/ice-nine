use anyhow::Result;
use async_trait::async_trait;
use notify::{recommended_watcher, EventHandler, Event, RecommendedWatcher, Watcher, RecursiveMode, EventKind};
use crb::agent::{Agent, AgentSession, Duty, Next, Address, OnEvent, Context};
use crb::core::Slot;
use std::path::PathBuf;
use std::time::Duration;
use derive_more::From;

const DEFAULT_PATH: &str = "ice9.toml";

pub struct ConfigLoader {
    path: PathBuf,
    watcher: Slot<RecommendedWatcher>,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self {
            path: DEFAULT_PATH.into(),
            watcher: Slot::empty(),
        }
    }
}

impl Agent for ConfigLoader {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Configure)
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
    async fn handle(&mut self, result: EventResult, _ctx: &mut Self::Context) -> Result<()> {
        let event = result?;
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                // TODO: Notify the supervisor.
            }
            _other => {
                // TODO: How to handle other methods? What if the config was removed?
            }
        }
        Ok(())
    }
}
