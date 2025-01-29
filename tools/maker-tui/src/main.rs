use anyhow::Result;
use crb::agent::{InteractiveTask, Runnable};
use ice9_maker_tui::AppTui;
use ui9_app::App;
use ui9_dui::Hub;

#[tokio::main]
async fn main() -> Result<()> {
    crb::agent::CRB.set_long_threshold(4_000);

    Hub::activate().await?;
    let (app, link) = App::new();
    let mut addr = app.spawn_connected();
    AppTui::new(link).run().await;

    env_logger::try_init()?;
    addr.interrupt()?;
    addr.join().await?;
    Hub::deactivate().await?;
    // Unblocking stdin
    std::process::exit(0);
}
