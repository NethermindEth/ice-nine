use anyhow::Result;
use crb::agent::Standalone;
use ice_nine_core::{Substance, SubstanceClient};
use ice_nine_plugin_telegram::TelegramParticle;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::try_init()?;
    let substance = Substance::new();
    let mut addr: SubstanceClient = substance.spawn().into();
    addr.add_particle::<TelegramParticle>()?;
    addr.join().await?;
    Ok(())
}
