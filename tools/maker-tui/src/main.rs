use anyhow::Result;
use crb::agent::Runnable;
use ice9_maker_tui::AppTui;

#[tokio::main]
async fn main() -> Result<()> {
    AppTui::new().run().await;
    // Unblocking stdin
    std::process::exit(0);
}
