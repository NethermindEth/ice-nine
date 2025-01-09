use crate::keeper::KeeperLink;
use crate::router::RouterLink;
use crb::agent::Agent;

pub struct ParticleSetup {
    pub keeper: KeeperLink,
    pub router: RouterLink,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
