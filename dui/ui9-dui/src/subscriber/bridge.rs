use std::sync::LazyLock;
use crb::core::{mpsc, sync::Mutex};
use crb::agent::{OnEvent, TheEvent, ToAddress};
use crate::subscriber::client::Delegate;

static SUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

pub struct EventBridge<T> {
    tx: mpsc::UnboundedSender<T>,
    rx: Mutex<Option<mpsc::UnboundedReceiver<T>>>,
}

impl<T> EventBridge<T>
where
    T: TheEvent,
{
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            tx,
            rx: Mutex::new(Some(rx)),
        }
    }

    pub fn subscribe<A>(&'static self, addr: impl ToAddress<A>)
    where
        A: OnEvent<T>,
    {
        let address = addr.to_address();
        crb::core::spawn(async move {
            let rx = self.rx.lock().await.take();
            if let Some(mut rx) = rx {
                while let Some(event) = rx.recv().await {
                    address.event(event).ok();
                }
            }
        });
    }
}
