use crate::keeper::{Config, KeeperLink};
use crate::router::{
    model::Model,
    tool::{CallParameters, Tool, ToolMeta},
    RouterLink,
};
use anyhow::Result;
use crb::agent::{Address, Agent};
use derive_more::{Deref, DerefMut};

#[derive(Clone)]
pub struct SubstanceLinks {
    pub keeper: KeeperLink,
    pub router: RouterLink,
}

#[derive(Deref, DerefMut)]
pub struct ParticleSetup {
    pub links: SubstanceLinks,
}

impl ParticleSetup {
    pub async fn config<C: Config>(&mut self) -> Result<C> {
        self.keeper.get_config().await
    }

    pub fn bond<A: Agent>(&mut self, address: Address<A>) -> SubstanceBond<A> {
        SubstanceBond {
            address,
            links: self.links.clone(),
        }
    }
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}

pub struct SubstanceBond<A: Agent> {
    address: Address<A>,
    links: SubstanceLinks,
}

impl<A: Agent> SubstanceBond<A> {
    pub fn add_model(&mut self) -> Result<()>
    where
        A: Model,
    {
        let address = self.address.clone();
        self.links.router.add_model(address)
    }

    pub async fn add_tool<P>(&mut self, tool: &A) -> Result<()>
    where
        A: Tool<P>,
        P: CallParameters,
    {
        let address = self.address.clone();
        let meta = ToolMeta {
            name: tool.name(),
            description: tool.description(),
            parameters: None,
        };
        self.links.router.add_tool(address, meta).await?;
        Ok(())
    }
}
