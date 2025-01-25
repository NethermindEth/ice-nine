use crate::connector::Connector;
use crate::publisher::{HubServer, HubServerLink};
use crate::relay::Relay;
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
    pub fn link() -> Result<&'static HubLink> {
        HUB.get().ok_or_else(|| anyhow!("Hub is not assigned"))
    }

    pub async fn activate() -> Result<()> {
        let hub = Hub::new();
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
        let connector = ctx.schedule(connector, ());

        let server = HubServer::new(connector.clone());
        let server = ctx.schedule(server, ());

        let client = HubClient::new(connector.clone());
        let client = ctx.schedule(client, ());

        let relay = Relay::new(connector.clone());
        let relay = ctx.schedule(relay, ());

        let link = HubLink {
            hub: ctx.to_address(),
            server: server.equip(),
            client: client.equip(),
        };
        HUB.set(link)
            .map_err(|_| anyhow!("Hub is already activated"))?;

        // Spawning the connector after the `Hub` is set, because it has peers tracer
        ctx.spawn_scheduled();

        Ok(Next::events())
    }
}
