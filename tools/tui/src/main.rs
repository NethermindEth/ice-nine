use anyhow::Result;
use crb::agent::Runnable;
use ui9_tool_tui::TuiApp;

#[tokio::main]
async fn main() -> Result<()> {
    TuiApp::new().run().await?;
    // Unblocking stdin
    std::process::exit(0);
}
