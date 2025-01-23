use crate::connector::Connector;
use crate::tracer::TracerInfo;
use crate::tracers::Tree;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Equip, Next, OnEvent, Standalone};
use crb::core::Slot;
use crb::runtime::Runtime;
use crb::superagent::{Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use std::sync::OnceLock;

static HUB: OnceLock<HubServerLink> = OnceLock::new();

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubServerLink {
    hub: Address<HubServer>,
}

impl HubServerLink {
    pub fn add_relay(&self, tracer_info: TracerInfo, runtime: impl Runtime) -> Result<()> {
        let delegate = Delegate {
            tracer_info,
            runtime: Box::new(runtime),
        };
        self.event(delegate)
    }
}

pub struct HubServer {
    /// `Tree` needs `HubServer` itself (uses `Tracer`), so it will be initialized after actor activation
    tree: Slot<Tree>,
}

impl HubServer {
    pub fn link() -> Option<&'static HubServerLink> {
        HUB.get()
    }

    pub fn activate() {
        let hub = HubServer {
            tree: Slot::empty(),
        };
        let address = hub.spawn().equip();
        if let Err(address) = HUB.set(address) {
            // Interrupt since hub is spawned already.
            address.interrupt();
        }
    }

    pub async fn deactivate() {
        if let Some(mut address) = HUB.get().cloned() {
            address.interrupt();
            address.join().await;
        }
    }
}

impl Standalone for HubServer {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    Connector,
    Relay,
}

impl Supervisor for HubServer {
    type GroupBy = Group;
}

impl Agent for HubServer {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for HubServer {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.tree.fill(Tree::new())?;

        let connector = Connector::new();
        ctx.spawn_agent(connector, Group::Connector);
        Ok(Next::events())
    }
}

pub struct Delegate {
    tracer_info: TracerInfo,
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for HubServer {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, Group::Relay);
        Ok(())
    }
}
