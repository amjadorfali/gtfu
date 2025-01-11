use anyhow::{Context, Result};
use clap::Parser;
use human_panic::setup_panic;
use log::info;

use std::io::Write;

use clap::Subcommand;
// use gtfu::Cli;
// check https://docs.rs/confy/latest/confy/index.html for managing configurations
// - Will automate the timer and break start/end. Need to have the ability to:
//  - Take the user input
//  - Start/Stop the timer
//  - Start/Skip/Reset a break
//
fn main() -> Result<(), String> {
    setup_panic!();
    env_logger::init();
    info!("starting up");

    // let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;

    loop {
        let line = readline("Please input a break interval in minutes: ")?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let cli = Cli2::try_parse_from(String::from(line).split(" ")).map_err(|e| e.to_string())?;

    println!("hey: {cli:?}");
    Ok(false)
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ping,
    Exit,
}
#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli2 {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn readline(msg: &str) -> Result<String, String> {
    write!(std::io::stdout(), "{msg}").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
