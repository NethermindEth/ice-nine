use crate::connector::Connector;
use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Deref, DerefMut, From};
use crb::agent::{Agent, Context, Duty, Next, Address};
use crb::superagent::{Supervisor, SupervisorSession};
use std::sync::OnceLock;

static CLIENT: OnceLock<HubClientLink> = OnceLock::new();

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubClientLink {
    hub: Address<HubClient>,
}

pub struct HubClient {}

impl HubClient {
    pub fn link() -> Option<&'static HubClientLink> {
        CLIENT.get()
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
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let connector = Connector::new();
        ctx.spawn_agent(connector, ());
        Ok(Next::events())
    }
}
