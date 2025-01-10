use anyhow::{anyhow, Result};
use crb::agent::{RunAgent, Task};
use crb::runtime::InteractiveRuntime;
use tokio::runtime::Runtime;
use ui9_tool_dashboard::{App, AppUi};

fn main() -> Result<()> {
    console_subscriber::init();
    let app = App::new();
    let runtime = RunAgent::new(app);
    let addr = runtime.address().clone();
    let handle = std::thread::spawn(|| {
        second_main(runtime);
    });
    AppUi::entrypoint(addr);
    handle
        .join()
        .map_err(|_| anyhow!("Can't get result of the thread."))?;
    Ok(())
}

fn second_main(runtime: RunAgent<App>) {
    Runtime::new().unwrap().block_on(runtime.run());
}
