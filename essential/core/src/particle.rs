use crate::keeper::{Config, KeeperLink};
use crate::router::RouterLink;
use anyhow::Result;
use crb::agent::Agent;
use derive_more::{Deref, DerefMut};
use std::marker::PhantomData;

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
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}

pub struct SubstanceBond<A: Agent> {
    _type: PhantomData<A>,
    links: SubstanceLinks,
}
