use anyhow::Result;
use crb::agent::{Equip, Standalone};
use ice_nine_core::{Substance, SubstanceClient};
use ice_nine_plugin_messaging_telegram::TelegramParticle;
use ice_nine_plugin_model_openai::OpenAIParticle;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    let substance = Substance::new();
    let mut addr: SubstanceClient = substance.spawn().equip();
    addr.add_particle::<OpenAIParticle>()?;
    addr.add_particle::<TelegramParticle>()?;
    addr.join().await?;
    Ok(())
}
