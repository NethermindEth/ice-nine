use anyhow::Result;
use crb_system::Main;
use ice9_core::Substance;
use ice_nine_plugin_chat_telegram::TelegramParticle;
use ice_nine_plugin_exchange_dydx::DyDxParticle;
use ice_nine_plugin_model_openai::OpenAIParticle;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Mesh::activate().await?;
    let substance = Substance::arise();
    substance.add_particle::<OpenAIParticle>()?;
    substance.add_particle::<DyDxParticle>()?;
    substance.add_particle::<TelegramParticle>()?;
    // Stdio is not compatible with tracing and will be replaced with DUI
    // substance.add_particle::<StdioParticle>()?;
    substance.into_address().join_or_signal().await?;
    Mesh::deactivate().await?;
    Ok(())
}
