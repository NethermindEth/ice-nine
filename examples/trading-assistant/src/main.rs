use anyhow::Result;
use ice9_core::Substance;
use ice_nine_plugin_chat_stdio::StdioParticle;
use ice_nine_plugin_chat_telegram::TelegramParticle;
use ice_nine_plugin_exchange_dydx::DyDxParticle;
use ice_nine_plugin_model_openai::OpenAIParticle;
use ui9_dui::HubServer;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    HubServer::activate();
    let mut substance = Substance::arise();
    substance.add_particle::<OpenAIParticle>()?;
    substance.add_particle::<DyDxParticle>()?;
    substance.add_particle::<TelegramParticle>()?;
    // Stdio is not compatible with tracing and will be replaced with DUI
    // substance.add_particle::<StdioParticle>()?;
    substance.join().await?;
    HubServer::deactivate().await;
    Ok(())
}
