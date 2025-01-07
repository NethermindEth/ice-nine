use crate::keeper::KeeperClient;
use crb::agent::Agent;

pub struct ParticleSetup {
    pub keeper: KeeperClient,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
