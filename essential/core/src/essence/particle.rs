use super::SubstanceLinks;
use crate::keeper::subscription::ConfigSegmentUpdates;
use crate::keeper::{subscription::UpdateConfig, Config};
use crate::router::{
    model::Model,
    tool::{CallParameters, Tool, ToolMeta},
};
use anyhow::Result;
use crb::agent::{Address, Agent, ToAddress};
use crb::superagent::Entry;

impl SubstanceLinks {
    pub async fn config<C: Config>(&mut self) -> Result<C> {
        self.keeper.get_config().await
    }

    pub fn bond<A: Agent>(&mut self, recipient: impl ToAddress<A>) -> SubstanceBond<A> {
        SubstanceBond {
            address: recipient.to_address(),
            substance: self.clone(),
        }
    }
}

pub trait Particle: Agent<Context: Default> {
    fn construct(substance: SubstanceLinks) -> Self;
}

pub struct SubstanceBond<A: Agent> {
    address: Address<A>,
    substance: SubstanceLinks,
}

impl<A: Agent> SubstanceBond<A> {
    pub async fn live_config_updates<C>(&mut self) -> Result<(C, Entry<ConfigSegmentUpdates>)>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let address = self.address.clone();
        let pair = self.substance.keeper.live_config_updates(address).await?;
        Ok(pair)
    }

    pub fn add_model(&mut self) -> Result<()>
    where
        A: Model,
    {
        let address = self.address.clone();
        self.substance.router.add_model(address)
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
        self.substance.router.add_tool(address, meta).await?;
        Ok(())
    }
}
