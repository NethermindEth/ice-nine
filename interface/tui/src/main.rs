use anyhow::Result;
use ui9_tool_tui::TuiApp;
use crb::agent::Runnable;

#[tokio::main]
async fn main() -> Result<()> {
    TuiApp::new().run().await?;
    // Unblocking stdin
    std::process::exit(0);
}
