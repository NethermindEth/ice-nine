pub mod encoding;
pub mod flow;
pub mod packed;

pub use flow::Flow;
pub use packed::{PackedAction, PackedEvent, PackedState};

use ui9::names::Fqn;

pub trait Unified: Flow {
    fn fqn() -> Fqn;
}
