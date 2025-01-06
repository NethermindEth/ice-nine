use crb::agent::Agent;
use crate::keeper::KeeperAddress;

pub struct ParticleSetup {
    pub keeper: KeeperAddress,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
