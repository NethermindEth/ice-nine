use crate::publisher::{HubServer, HubServerLink};
use crate::reporter::Reporter;
use crate::subscriber::{HubClient, HubClientLink, PlayerGenerator};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, DoAsync, Equip, Next, Standalone, ToAddress,
};
use crb::superagent::{PingExt, Stacker, Supervisor, SupervisorSession};
use std::sync::OnceLock;

static HUB: OnceLock<HubLink> = OnceLock::new();

pub struct HubLink {
    pub hub: Address<Hub>,
    pub server: HubServerLink,
    pub client: HubClientLink,
}

pub struct Hub {}

impl Hub {
    pub fn link() -> Result<&'static HubLink> {
        HUB.get().ok_or_else(|| anyhow!("Hub is not assigned"))
    }

    pub async fn activate<G>(generator: G) -> Result<()>
    where
        G: PlayerGenerator,
    {
        let hub = Self::new();
        hub.spawn().ping().await?;
        Ok(())
    }

    pub async fn deactivate() -> Result<()> {
        if let Some(link) = HUB.get() {
            let mut hub = link.hub.clone();
            hub.interrupt()?;
            hub.join().await?;
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Standalone for Hub {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    Client,
    Server,
    Reporter,
}

impl Supervisor for Hub {
    type BasedOn = AgentSession<Self>;
    type GroupBy = Group;
}

impl Agent for Hub {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Hub {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut stacker = Stacker::new();

        let server = HubServer::new();
        let server = stacker.schedule(server, Group::Server);

        let client = HubClient::new();
        let client = stacker.schedule(client, Group::Client);

        let reporter = Reporter::new();
        stacker.schedule(reporter, Group::Reporter);

        let link = HubLink {
            hub: ctx.to_address(),
            server: server.equip(),
            client: client.equip(),
        };
        HUB.set(link)
            .map_err(|_| anyhow!("Hub is already activated"))?;

        // Spawning the connector after the `Hub` is set, because it has peers tracer
        stacker.spawn_scheduled(ctx);

        Ok(Next::events())
    }
}
