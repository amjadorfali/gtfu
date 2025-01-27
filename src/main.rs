use std::{
    io::{stdout, Write},
    ops::{Sub, SubAssign},
    thread,
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use env_logger::{Env, DEFAULT_FILTER_ENV};
use human_panic::setup_panic;
use log::info;

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
    let sleep_duration = Duration::new(1, 0);
    let mut break_time = args.freq.clone();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        stdout().flush()?;
        print!("\rNext break in: {} seconds", break_time.as_secs());
        if break_time.is_zero() {
            println!("\rhorrayyyyy!!! Go get a life now");
            return Ok(());
        }
        break_time.sub_assign(sleep_duration);
        thread::sleep(sleep_duration);
    }
}
