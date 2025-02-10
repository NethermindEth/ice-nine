use anyhow::Result;
use n9_core::Substance;
// use ice_nine_plugin_chat_telegram::TelegramParticle;
use ice_nine_plugin_exchange_dydx::DyDxParticle;
// use ice_nine_plugin_model_anthropic::AnthropicParticle;
use ice_nine_plugin_model_openai::OpenAIParticle;
use n9_app_stdio::StdioApp;
use n9_control_chat::ChatParticle;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Mesh::activate().await?;
    let mut substance = Substance::arise();
    substance.add_particle::<OpenAIParticle>()?;
    substance.add_particle::<DyDxParticle>()?;
    substance.add_particle::<ChatParticle>()?;
    substance.add_particle::<StdioApp>()?;

    // substance.add_particle::<AnthropicParticle>()?;

    // substance.add_particle::<TelegramParticle>()?;
    // Stdio is not compatible with tracing and will be replaced with DUI
    // substance.add_particle::<StdioParticle>()?;
    substance.join().await?;
    Mesh::deactivate().await?;
    // Unblocking stdin
    std::process::exit(0);
}
