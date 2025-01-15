use super::SubstanceLinks;
use crate::keeper::{updates::UpdateConfig, Config};
use crate::router::{
    model::Model,
    tool::{CallParameters, Tool, ToolMeta},
};
use anyhow::Result;
use crb::agent::{Address, Agent, ToAddress};
use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
pub struct ParticleSetup {
    pub links: SubstanceLinks,
}

impl ParticleSetup {
    pub async fn config<C: Config>(&mut self) -> Result<C> {
        self.keeper.get_config().await
    }

    pub fn bond<A: Agent>(&mut self, recipient: impl ToAddress<A>) -> SubstanceBond<A> {
        SubstanceBond {
            address: recipient.to_address(),
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
    pub async fn live_config_updates<C>(&mut self) -> Result<()>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let address = self.address.clone();
        let namespace = C::NAMESPACE.to_string();
        // TODO: Return a config
        self.links.keeper.subscribe(address, namespace).await?;
        Ok(())
    }

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
