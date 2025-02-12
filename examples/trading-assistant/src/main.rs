use anyhow::Result;
use n9_chat_telegram::TelegramParticle;
use n9_core::Substance;
use n9_exchange_dydx::DyDxParticle;
// use n9_model_anthropic::AnthropicParticle;
use n9_app_stdio::StdioApp;
use n9_app_tui::TuiApp;
use n9_control_chat::ChatParticle;
use n9_model_openai::OpenAIParticle;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init()?;
    Mesh::activate().await?;
    let mut substance = Substance::arise();
    // TODO: Rename to *Model
    substance.add_particle::<OpenAIParticle>()?;
    // substance.add_particle::<AnthropicParticle>()?;

    // TODO: Rename to *Exchange
    substance.add_particle::<DyDxParticle>()?;

    // TODO: Rename to *Control
    substance.add_particle::<ChatParticle>()?;

    // substance.add_particle::<StdioApp>()?;

    substance.add_particle::<TuiApp>()?;

    // TODO: Rename to *Chat
    substance.add_particle::<TelegramParticle>()?;

    // Stdio is not compatible with tracing and will be replaced with DUI
    // substance.add_particle::<StdioParticle>()?;
    substance.join().await?;
    Mesh::deactivate().await?;
    // Unblocking stdin
    std::process::exit(0);
}
