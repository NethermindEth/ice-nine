use crate::keeper::KeeperLink;
use crate::router::RouterLink;
use crb::agent::Agent;

#[derive(Clone)]
pub struct SubstanceLinks {
    pub keeper: KeeperLink,
    pub router: RouterLink,
}

pub struct ParticleSetup {
    pub links: SubstanceLinks,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
