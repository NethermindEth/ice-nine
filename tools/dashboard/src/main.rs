use anyhow::Result;
use crb::agent::Standalone;
use ui9_tool_dashboard::{App, AppUi};

fn main() -> Result<()> {
    std::thread::spawn(second_main);
    AppUi::entrypoint();
    Ok(())
}

#[tokio::main]
async fn second_main() -> Result<()> {
    let mut addr = App::new().spawn();
    addr.join().await?;
    Ok(())
}
