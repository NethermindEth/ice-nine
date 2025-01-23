use crate::connector::Connector;
use crate::publisher::{HubServer, HubServerLink};
use crate::subscriber::{HubClient, HubClientLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Equip, Next, Standalone, ToAddress};
use crb::superagent::{PingExt, Supervisor, SupervisorSession};
use std::sync::OnceLock;

static HUB: OnceLock<HubLink> = OnceLock::new();

pub struct HubLink {
    pub hub: Address<Hub>,
    pub server: HubServerLink,
    pub client: HubClientLink,
}

pub struct Hub {}

impl Hub {
    pub fn link() -> Option<&'static HubLink> {
        HUB.get()
    }

    pub async fn activate() -> Result<()> {
        let hub = Hub::new();
        hub.spawn().ping().await?;
        Ok(())
    }

    pub async fn deactivate() -> Result<()> {
        if let Some(mut link) = HUB.get() {
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

impl Supervisor for Hub {
    type GroupBy = ();
}

impl Agent for Hub {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::duty(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl Duty<Initialize> for Hub {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let connector = Connector::new();
        ctx.spawn_agent(connector, ());

        let server = HubServer::new();
        let server = ctx.spawn_agent(server, ());

        let client = HubClient::new();
        let client = ctx.spawn_agent(client, ());

        let link = HubLink {
            hub: ctx.to_address(),
            server: server.equip(),
            client: client.equip(),
        };
        HUB.set(link)
            .map_err(|_| anyhow!("Hub is already activated"))?;

        Ok(Next::events())
    }
}
