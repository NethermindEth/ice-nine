use anyhow::Result;
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        None => {}
        Some(Commands::Maker) => {}
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Maker,
}
