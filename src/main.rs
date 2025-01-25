use anyhow::{Context, Result};
use clap::Parser;
use env_logger::{Env, DEFAULT_FILTER_ENV};
use human_panic::setup_panic;
use log::{info, log};

use gtfu::Cli;
// check https://docs.rs/confy/latest/confy/index.html for managing configurations
// - Will automate the timer and break start/end. Need to have the ability to:
//  - Take the user input
//  - Start/Stop the timer
//  - Start/Skip/Reset a break
//
fn main() -> Result<()> {
    setup_panic!();
    let env = Env::default().filter_or(DEFAULT_FILTER_ENV, "gtfu");
    env_logger::init_from_env(env);
    info!("starting up");

    let args = Cli::parse();
    println!("args are {args:?}");
    // let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;

    Ok(())
}
