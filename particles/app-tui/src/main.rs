use anyhow::Result;
use crb::agent::{InteractiveTask, Runnable};
use n9_app_tui::AppTui;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    crb::agent::CRB.set_long_threshold(4_000);

    Mesh::activate().await?;
    AppTui::new().run().await;
    env_logger::try_init()?;
    Mesh::deactivate().await?;
    // Unblocking stdin
    std::process::exit(0);
}
