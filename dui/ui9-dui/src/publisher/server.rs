use crate::connector::Connector;
use crate::publisher::{RecorderLink, TracerInfo, UniRecoder};
use crate::tracers::Tree;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next, OnEvent, Standalone};
use crb::core::Slot;
use crb::runtime::{InteractiveRuntime, ReachableContext, Runtime};
use crb::superagent::{OnRequest, Relation, Request, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use std::collections::HashMap;
use ui9::names::Fqn;

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubServerLink {
    hub: Address<HubServer>,
}

impl HubServerLink {
    pub fn add_recorder<R>(&self, tracer_info: TracerInfo, runtime: R) -> Result<()>
    where
        R: InteractiveRuntime,
        <R::Context as ReachableContext>::Address: UniRecoder,
    {
        let delegate = Delegate {
            tracer_info,
            link: RecorderLink::new(runtime.address().clone()),
            runtime: Box::new(runtime),
        };
        self.event(delegate)
    }
}

pub struct HubServer {
    /// `Tree` needs `HubServer` itself (uses `Tracer`), so it will be initialized after actor activation
    tree: Slot<Tree>,
    connector: Address<Connector>,
    recorders: HashMap<Fqn, Record>,
    relations: HashMap<Relation<Self>, Fqn>,
}

impl HubServer {
    pub fn new(connector: Address<Connector>) -> Self {
        Self {
            tree: Slot::empty(),
            connector,
            recorders: HashMap::new(),
            relations: HashMap::new(),
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

    fn finished(&mut self, rel: &Relation<Self>, ctx: &mut Context<Self>) {
        if let Some(flow_id) = self.relations.remove(rel) {
            self.recorders.remove(&flow_id);
            // TODO: Remove from the tree as well
        }
    }
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
    async fn handle(&mut self, _: Initialize, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.tree.fill(Tree::new())?;

        Ok(Next::events())
    }
}

struct Record {
    tracer_info: TracerInfo,
    link: RecorderLink,
}

pub struct Delegate {
    tracer_info: TracerInfo,
    link: RecorderLink,
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for HubServer {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        let fqn = delegate.tracer_info.fqn.clone();
        // TODO: Check it doesn't exist
        let rel = ctx.spawn_trackable(delegate.runtime, Group::Relay);
        self.relations.insert(rel, fqn.clone());
        let record = Record {
            tracer_info: delegate.tracer_info,
            link: delegate.link,
        };
        self.recorders.insert(fqn.clone(), record);
        // TODO: Add to the aliases tree
        Ok(())
    }
}

pub struct Discover {
    fqn: Fqn,
}

impl Request for Discover {
    type Response = RecorderLink;
}

impl OnRequest<Discover> for HubServer {}
