use crate::publisher::HubServerLink;
use crate::subscriber::HubClientLink;
use anyhow::{anyhow, Result};
use crb::agent::Address;
use std::sync::OnceLock;

static HUB: OnceLock<HubLink> = OnceLock::new();

pub struct HubLink {
    pub server: HubServerLink,
    pub client: HubClientLink,
}

pub struct Hub {}

impl Hub {
    pub fn link() -> Option<&'static HubLink> {
        HUB.get()
    }

    pub async fn activate() -> Result<()> {
        Ok(())
        /*
        let hub = HubServer::new();
        let address = hub.spawn().equip();
        if let Err(address) = HUB.set(address) {
            // Interrupt since hub is spawned already.
            address.interrupt()?;
            Err(anyhow!("Hub is already activated"))
        } else {
            Ok(())
        }
        */
    }

    pub async fn deactivate() -> Result<()> {
        Ok(())
        /*
        if let Some(mut address) = HUB.get().cloned() {
            address.interrupt()?;
            address.join().await?;
        }
        Ok(())
        */
    }
}
