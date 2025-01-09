use crate::keeper::KeeperClient;
use crate::router::RouterClient;
use crb::agent::Agent;

pub struct ParticleSetup {
    pub keeper: KeeperClient,
    pub router: RouterClient,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
