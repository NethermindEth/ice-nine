use crate::connector::{Connector, ConnectorLink};
use crate::publisher::{HubServer, HubServerLink};
// use crate::relay::Relay;
use crate::subscriber::{HubClient, HubClientLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Duty, Equip, Next, Standalone, ToAddress};
use crb::superagent::{PingExt, Stacker, Supervisor, SupervisorSession};
use std::sync::OnceLock;

static HUB: OnceLock<HubLink> = OnceLock::new();

pub struct HubLink {
    pub hub: Address<Hub>,
    pub server: HubServerLink,
    pub client: HubClientLink,
    pub connector: ConnectorLink,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    // Relay,
    Client,
    Connector,
    Server,
}

impl Supervisor for Hub {
    type GroupBy = Group;
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
        let mut stacker = Stacker::new();

        let connector = Connector::new();
        let connector = stacker.schedule(connector, Group::Connector);

        let server = HubServer::new(connector.clone());
        let server = stacker.schedule(server, Group::Server);

        let client = HubClient::new(connector.clone());
        let client = stacker.schedule(client, Group::Client);

        /*
        let relay = Relay::new(connector.clone());
        let relay = stacker.schedule(relay, Group::Relay);
        */

        let link = HubLink {
            hub: ctx.to_address(),
            server: server.equip(),
            client: client.equip(),
            connector: connector.equip(),
        };
        HUB.set(link)
            .map_err(|_| anyhow!("Hub is already activated"))?;

        // Spawning the connector after the `Hub` is set, because it has peers tracer
        stacker.spawn_scheduled(ctx);

        Ok(Next::events())
    }
}
