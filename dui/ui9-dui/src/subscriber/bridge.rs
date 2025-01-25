use std::sync::LazyLock;
use crb::core::{mpsc, sync::Mutex};
use crate::subscriber::client::Delegate;

static SUB_BRIDGE: LazyLock<SubBridge<Delegate>> = LazyLock::new(|| SubBridge::new());

pub struct SubBridge<T> {
    tx: mpsc::UnboundedSender<T>,
    rx: Mutex<Option<mpsc::UnboundedReceiver<T>>>,
}

impl<T> SubBridge<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            tx,
            rx: Mutex::new(Some(rx)),
        }
    }
}
