use crb::agent::Agent;
use crate::keeper::KeeperClient;

pub struct ParticleSetup {
    pub keeper: KeeperClient,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
