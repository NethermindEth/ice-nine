use anyhow::Result;
use crb::agent::{Equip, Standalone};
use ice_nine_core::{Substance, SubstanceLink};
use ice_nine_plugin_exchange_dydx::DyDxParticle;
use ice_nine_plugin_messaging_telegram::TelegramParticle;
use ice_nine_plugin_model_openai::OpenAIParticle;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    let substance = Substance::new();
    let mut addr: SubstanceLink = substance.spawn().equip();
    addr.add_particle::<OpenAIParticle>()?;
    addr.add_particle::<DyDxParticle>()?;
    addr.add_particle::<TelegramParticle>()?;
    addr.join().await?;
    Ok(())
}
