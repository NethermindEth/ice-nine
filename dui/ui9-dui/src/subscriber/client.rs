use crate::connector::Connector;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Next, OnEvent};
use crb::runtime::Runtime;
use crb::superagent::{Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubClientLink {
    hub: Address<HubClient>,
}

impl HubClientLink {
    pub fn add_player(&self, runtime: impl Runtime) -> Result<()> {
        let delegate = Delegate {
            runtime: Box::new(runtime),
        };
        self.event(delegate)
    }
}

pub struct HubClient {
    connector: Address<Connector>,
}

impl HubClient {
    pub fn new(connector: Address<Connector>) -> Self {
        Self { connector }
    }
}

impl Supervisor for HubClient {
    type GroupBy = ();
}

impl Agent for HubClient {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for HubClient {
    async fn handle(&mut self, _: Initialize, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}

pub struct Delegate {
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for HubClient {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, ());
        Ok(())
    }
}
