use anyhow::{anyhow, Result};
use crb::agent::RunAgent;
use crb::runtime::InteractiveRuntime;
use ice9_maker::{App, AppUi};
use tokio::runtime::Runtime;
use ui9_dui::Hub;

fn main() -> Result<()> {
    env_logger::try_init()?;
    let app = App::new();
    let runtime = RunAgent::new(app);
    let addr = runtime.address().clone();
    let handle = std::thread::spawn(|| -> Result<()> {
        let fut = second_main(runtime);
        Runtime::new()?.block_on(fut)?;
        Ok(())
    });
    AppUi::entrypoint(addr);
    handle
        .join()
        .map_err(|_| anyhow!("Can't get result of the thread."))??;
    std::process::exit(0);
}

async fn second_main(runtime: RunAgent<App>) -> Result<()> {
    Hub::activate().await?;
    runtime.await;
    Hub::deactivate().await?;
    Ok(())
}
