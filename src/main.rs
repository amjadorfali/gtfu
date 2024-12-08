use anyhow::{Context, Result};
use clap::Parser;
use human_panic::setup_panic;
use log::info;

use gtfu::Cli;
// check https://docs.rs/confy/latest/confy/index.html for managing configurations
fn main() -> Result<()> {
    setup_panic!();
    env_logger::init();
    info!("starting up");
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    Ok(())
}
