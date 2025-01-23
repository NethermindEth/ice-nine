use clap::Parser;

#[derive(Parser, Clone)]
pub struct RunArgs {
    pub command: String,
    pub arguments: Vec<String>,
}
