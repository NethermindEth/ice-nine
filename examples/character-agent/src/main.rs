use anyhow::Result;
use crb::agent::Standalone;
use ice_nine_core::substance::Substance;

#[tokio::main]
async fn main() -> Result<()> {
    let substance = Substance::new();
    let mut addr = substance.spawn();
    addr.join().await?;
    Ok(())
}
