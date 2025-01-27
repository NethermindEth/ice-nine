mod recorder;
mod server;
mod tracer;

pub use recorder::{EventFlow, Recorder, RecorderLink, UniRecorder};
pub use server::{HubServer, HubServerLink};
pub use tracer::{Tracer, TracerInfo};

use crate::flow::Flow;
use crb::core::mpsc;

pub struct RecorderSetup<F: Flow> {
    state: F,
    action_tx: mpsc::UnboundedSender<F::Action>,
}
