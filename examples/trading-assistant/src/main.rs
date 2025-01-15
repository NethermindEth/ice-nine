use anyhow::Result;
use ice_nine_core::Substance;
use ice_nine_plugin_exchange_dydx::DyDxParticle;
use ice_nine_plugin_chat_stdio::StdioParticle;
use ice_nine_plugin_chat_telegram::TelegramParticle;
use ice_nine_plugin_model_openai::OpenAIParticle;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    let mut substance = Substance::arise();
    substance.add_particle::<OpenAIParticle>()?;
    substance.add_particle::<DyDxParticle>()?;
    substance.add_particle::<TelegramParticle>()?;
    substance.add_particle::<StdioParticle>()?;
    substance.join().await?;
    Ok(())
}
