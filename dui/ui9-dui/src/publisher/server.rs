use crate::connector::Connector;
use crate::publisher::TracerInfo;
use crate::tracers::Tree;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Equip, Next, OnEvent, Standalone};
use crb::core::Slot;
use crb::runtime::Runtime;
use crb::superagent::{Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use std::sync::OnceLock;

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubServerLink {
    hub: Address<HubServer>,
}

impl HubServerLink {
    pub fn add_recorder(&self, tracer_info: TracerInfo, runtime: impl Runtime) -> Result<()> {
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
    connector: Address<Connector>,
}

impl HubServer {
    pub fn new(connector: Address<Connector>) -> Self {
        Self {
            tree: Slot::empty(),
            connector,
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
