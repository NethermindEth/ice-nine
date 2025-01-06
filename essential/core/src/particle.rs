use crb::agent::Agent;
use crate::keeper::KeeperAddress;

pub struct ParticleSetup {
    pub keeper: KeeperAddress,
}

pub trait Particle: Agent {
    fn construct(setup: ParticleSetup) -> Self;
}
