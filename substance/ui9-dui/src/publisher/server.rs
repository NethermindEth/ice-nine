use super::{Pub, Recorder, RecorderLink, RecorderState, TracerInfo};
use crate::flow::Flow;
use crate::tracers::tree::Tree;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, Context, DoAsync, ManagedContext, Next, OnEvent, RunAgent, Standalone,
    StopAddress,
};
use crb::runtime::{InteractiveRuntime, Runtime};
use crb::superagent::{
    EventBridge, InteractExt, OnRequest, Relation, Request, StreamSession, Supervisor,
    SupervisorSession,
};
use derive_more::{Deref, DerefMut, From};
use std::collections::HashMap;
use std::sync::LazyLock;
use ui9::names::Fqn;

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubServerLink {
    hub: Address<HubServer>,
}

impl HubServerLink {
    pub async fn discover(&self, fqn: Fqn) -> Result<RecorderLink> {
        let msg = Discover { fqn };
        let link = self.hub.interact(msg).await?;
        Ok(link)
    }
}

static PUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

impl HubServer {
    pub fn spawn_recorder<F>(fqn: Fqn, state: RecorderState<F>) -> StopAddress<Recorder<F>>
    where
        F: Flow,
    {
        let recorder = Recorder::new(state);
        let runtime = RunAgent::new(recorder);
        let address = runtime.address();
        let tracer_info = TracerInfo {
            class: F::class().into(),
        };
        let delegate = Delegate {
            fqn,
            tracer_info,
            link: RecorderLink::new(runtime.address().clone()),
            runtime: Box::new(runtime),
        };
        PUB_BRIDGE.send(delegate);
        address.to_stop_address()
    }
}

pub struct HubServer {
    /// `Tree` needs `HubServer` itself (uses `Tracer`), so it will be initialized after actor activation
    tree: Option<Pub<Tree>>,
    recorders: HashMap<Fqn, RecorderLink>,
    relations: HashMap<Relation<Self>, Fqn>,
}

impl HubServer {
    pub fn new() -> Self {
        Self {
            tree: None,
            recorders: HashMap::new(),
            relations: HashMap::new(),
        }
    }
}

impl Standalone for HubServer {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    Connector,
    Relay,
}

impl Supervisor for HubServer {
    type BasedOn = StreamSession<Self>;
    type GroupBy = Group;

    fn finished(&mut self, rel: &Relation<Self>, _ctx: &mut Context<Self>) {
        if let Some(fqn) = self.relations.remove(rel) {
            self.recorders.remove(&fqn);
            if let Some(tree) = self.tree.as_mut() {
                tree.del(fqn);
            }
        }
    }
}

impl Agent for HubServer {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }

    fn interrupt(&mut self, ctx: &mut Context<Self>) {
        self.tree.take();
        ctx.shutdown();
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for HubServer {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        log::debug!("HubServer starting...");
        ctx.consume(PUB_BRIDGE.events().await?);
        self.tree = Some(Pub::unified());
        log::debug!("HubServer active");

        Ok(Next::events())
    }
}

pub struct Delegate {
    fqn: Fqn,
    tracer_info: TracerInfo,
    link: RecorderLink,
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for HubServer {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        let fqn = delegate.fqn;
        if !self.recorders.contains_key(&fqn) {
            let rel = ctx.spawn_trackable(delegate.runtime, Group::Relay);
            self.relations.insert(rel, fqn.clone());
            self.recorders.insert(fqn.clone(), delegate.link);
            if let Some(tree) = self.tree.as_mut() {
                tree.add(fqn, delegate.tracer_info);
            }
            Ok(())
        } else {
            Err(anyhow!("Recorder {fqn} already registered"))
        }
    }
}

pub struct Discover {
    fqn: Fqn,
}

impl Request for Discover {
    type Response = RecorderLink;
}

#[async_trait]
impl OnRequest<Discover> for HubServer {
    async fn on_request(
        &mut self,
        req: Discover,
        _ctx: &mut Context<Self>,
    ) -> Result<RecorderLink> {
        self.recorders
            .get(&req.fqn)
            .cloned()
            .ok_or_else(|| anyhow!("Recorder {} not found", req.fqn))
    }
}
