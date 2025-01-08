use crate::conversation_router::ConversationRouterClient;
use crate::keeper::KeeperClient;
use crb::agent::Agent;

pub struct ParticleSetup {
    pub keeper: KeeperClient,
    pub conversation_router: ConversationRouterClient,
}

pub trait Particle: Agent<Context: Default> {
    fn construct(setup: ParticleSetup) -> Self;
}
