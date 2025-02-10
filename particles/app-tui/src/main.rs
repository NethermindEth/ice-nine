use anyhow::Result;
use crb::agent::{InteractiveTask, Runnable};
use n9_app_tui::AppTui;
use ui9_maker::App;
use ui9_mesh::Mesh;

#[tokio::main]
async fn main() -> Result<()> {
    crb::agent::CRB.set_long_threshold(4_000);

    Mesh::activate().await?;
    let (app, link) = App::new();
    let mut addr = app.spawn_connected();
    AppTui::new(link).run().await;

    env_logger::try_init()?;
    addr.interrupt()?;
    addr.join().await?;
    Mesh::deactivate().await?;
    // Unblocking stdin
    std::process::exit(0);
}
