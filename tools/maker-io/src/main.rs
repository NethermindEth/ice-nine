use anyhow::{anyhow, Result};
use clap::Parser;
use crb::agent::{RunAgent, Task};
use crb::runtime::InteractiveRuntime;
use ice9_maker_io::{App, AppUi, RunArgs};
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    // console_subscriber::init();
    env_logger::try_init()?;
    let args = RunArgs::parse();
    let (app, rx) = App::new(args);
    let runtime = RunAgent::new(app);
    let addr = runtime.address().clone();
    let handle = std::thread::spawn(|| {
        second_main(runtime);
    });
    AppUi::entrypoint(addr, rx);
    handle
        .join()
        .map_err(|_| anyhow!("Can't get result of the thread."))?;
    std::process::exit(0);
}

fn second_main(runtime: RunAgent<App>) {
    Runtime::new().unwrap().block_on(runtime.run());
}
