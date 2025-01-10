use crate::keeper::{Config, KeeperLink};
use crate::router::RouterLink;
use anyhow::Result;
use crb::agent::Agent;

#[derive(Clone)]
pub struct SubstanceLinks {
    pub keeper: KeeperLink,
    pub router: RouterLink,
}

impl SubstanceLinks {
    pub async fn config<C: Config>(&mut self) -> Result<C> {
        self.keeper.get_config().await
    }
}

pub struct ParticleSetup {
    pub links: SubstanceLinks,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
