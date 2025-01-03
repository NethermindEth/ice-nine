pub mod add;
pub mod lookup;
pub mod remove;

use crb::agent::{Agent, AgentSession};
use typedmap::TypedDashMap;

pub struct Registry {
    links: TypedDashMap,
}

impl Agent for Registry {
    type Context = AgentSession<Self>;
    type Output = ();
}
