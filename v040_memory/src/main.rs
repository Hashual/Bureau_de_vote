use clap::Parser;
use v040_memory::{app_builder::run_app, configuration::Configuration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}