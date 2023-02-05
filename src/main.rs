use std::path::Path;
use anyhow::Result;
use clap::Parser;
use crate::expander::Expander;
use crate::packages::SentryCliPackage;

mod packages;
mod expander;
mod utils;

#[derive(Parser)]
struct Args {
    #[arg(index = 1)]
    target: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let args: Args = Args::parse();
    let expander = Expander::new(Path::new(&args.target));

    expander.expand(&SentryCliPackage::new()).await?;

    Ok(())
}