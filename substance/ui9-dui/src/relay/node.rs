use super::connector::{Connector, ConnectorLink};
use super::router::Router;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, DoAsync, Equip, Next, Standalone, ToAddress,
};
use crb::superagent::{PingExt, Supervisor, SupervisorSession};

use std::sync::OnceLock;

// TODO: Swam Connector/Router roles: Router has to spawn a Connector
static NODE: OnceLock<MeshNodeLink> = OnceLock::new();

pub struct MeshNodeLink {
    pub node: Address<MeshNode>,
    pub connector: ConnectorLink,
}

pub struct MeshNode {}

impl MeshNode {
    pub fn link() -> Result<&'static MeshNodeLink> {
        NODE.get()
            .ok_or_else(|| anyhow!("MeshNode is not assigned"))
    }

    pub async fn activate() -> Result<()> {
        let connector = Self::new();
        connector.spawn().ping().await?;
        Ok(())
    }

    pub async fn deactivate() -> Result<()> {
        if let Some(link) = NODE.get() {
            let mut connector = link.connector.clone();
            connector.interrupt()?;
            connector.join().await?;
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Standalone for MeshNode {}

impl Supervisor for MeshNode {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for MeshNode {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for MeshNode {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let connector = Connector::new();
        let connector: ConnectorLink = ctx.spawn_agent(connector, ()).equip();

        let router = Router::new(connector.clone());
        ctx.spawn_agent(router, ());

        let link = MeshNodeLink {
            node: ctx.to_address(),
            connector,
        };
        NODE.set(link)
            .map_err(|_| anyhow!("Node is already activated"))?;

        Ok(Next::events())
    }
}
