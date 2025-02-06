use std::{
    io::{stdin, stdout, Write},
    ops::SubAssign,
    process::Command,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use anyhow::{Error, Result as Result_anyhow};
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

// TODO: refactor threads into async tasks

fn main() -> Result_anyhow<()> {
    setup_panic!();
    let env = Env::default().filter_or(DEFAULT_FILTER_ENV, "gtfu");
    env_logger::init_from_env(env);
    info!("starting up");

    let args = Cli::parse();
    if let Some(reset) = args.reset {
        if args.freq.le(&reset.into()) {
            return Err(Error::msg(
                "Idle reset time cannot be greater than or equals to the break time",
            ));
        }
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("You can modify the break/timer using:");
    println!("N: Next -- P: Pause/Play -- R: Reset -- E: Exit");
    let (sender, reciever) = channel::<String>();
    thread::spawn(move || capture_interrupts(sender));
    thread::spawn(move || run_break(args, reciever))
        .join()
        .unwrap()?;

    Ok(())
}

fn run_break(cli_args: Cli, receiver: Receiver<String>) -> Result_anyhow<()> {
    let sleep_duration = Duration::new(1, 0);
    let mut paused = false;
    let mut break_freq = cli_args.freq.clone();
    let mut break_length = cli_args.len.clone();
    let mut is_break = false;
    loop {
        match receiver.try_recv() {
            Ok(interrupt) => match interrupt.as_str() {
                "N" | "n" => {
                    is_break = !is_break;
                    break_freq = cli_args.freq.clone();
                    break_length = cli_args.len.clone();
                }
                "P" | "p" => paused = !paused,
                "R" | "r" => {
                    break_freq = cli_args.freq.clone();
                    break_length = cli_args.len.clone();
                }
                "E" | "e" => return Ok(()),
                _ => {}
            },
            _ => {}
        }
        if is_break {
            if !paused {
                break_length.sub_assign(sleep_duration);
            }
            print!(
                "\rRelax your anus - Rest ends in: {} seconds",
                break_length.as_secs()
            );
            stdout().flush()?;
            if break_length.is_zero() {
                break_length = cli_args.len.clone();
                is_break = false;
            }
        } else {
            if !paused {
                break_freq.sub_assign(sleep_duration);
            }
            print!("\rNext break in: {} seconds", break_freq.as_secs());
            stdout().flush()?;
            if break_freq.is_zero() {
                for num in 3..1 {
                    print!("\rBreak starting in: {num}",);
                    stdout().flush()?;
                    thread::sleep(sleep_duration);
                }

                break_freq = cli_args.freq.clone();
                is_break = true;
            }
        }

        thread::sleep(sleep_duration);
    }
}

fn capture_interrupts(sender: Sender<String>) {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        sender.send(line.trim().to_string()).unwrap();
    }
}

#[cfg(target_os = "macos")]
fn get_idle_time() -> Option<Duration> {
    let output = Command::new("ioreg")
        .arg("-c")
        .arg("IOHIDSystem")
        .arg("-r")
        .arg("-k")
        .arg("HIDIdleTime")
        .output()
        .ok()?;
    let stdout = String::from_utf8(output.stdout).ok()?;
    let millis = stdout
        .lines()
        .find(|line| line.contains("HIDIdleTime"))?
        .split('=')
        .last()?
        .trim()
        .parse::<u64>()
        .ok()?;
    Some(Duration::from_nanos(millis))
}

#[cfg(target_os = "linux")]
fn get_idle_time() -> Option<Duration> {
    let output = Command::new("xprintidle").output().ok()?;
    let millis = String::from_utf8(output.stdout)
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()?;
    Some(Duration::from_millis(millis))
}
