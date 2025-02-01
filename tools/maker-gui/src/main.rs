use anyhow::{anyhow, Result};
use crb::agent::RunAgent;
use ice9_maker_gui::AppGui;
use tokio::runtime::Runtime;
use ui9_app::App;
use ui9_mesh::Mesh;

fn main() -> Result<()> {
    env_logger::try_init()?;
    let (app, link) = App::new();
    let handle = std::thread::spawn(|| -> Result<()> {
        let fut = second_main(app);
        Runtime::new()?.block_on(fut)?;
        Ok(())
    });
    AppGui::entrypoint(link);
    handle
        .join()
        .map_err(|_| anyhow!("Can't get result of the thread."))??;
    std::process::exit(0);
}

async fn second_main(runtime: RunAgent<App>) -> Result<()> {
    Mesh::activate().await?;
    runtime.await;
    Mesh::deactivate().await?;
    Ok(())
}
