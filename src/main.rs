use std::{
    io::{stdin, stdout, Write},
    ops::SubAssign,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use anyhow::{Error, Result as Result_anyhow};
use break_window::{
    window::{get_app, get_event_loop, CustomEvent},
    winit_app::run_app,
};

use clap::Parser;
use env_logger::{Env, DEFAULT_FILTER_ENV};
mod cli_parser;
use cli_parser::Cli;
use human_panic::setup_panic;
use log::info;

use idle_detection::idle::get_idle_time;
mod idle_detection {
    pub mod idle;
    #[cfg(target_os = "linux")]
    pub mod wayland_idle;
}

mod break_window {
    pub mod window;
    pub mod winit_app;
}

fn main() -> Result_anyhow<()> {
    setup_panic!();
    procspawn::init();
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
    // Clear terminal
    //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("You can modify the break/timer using:");
    println!("N: Next -- R: Reset -- E: Exit");
    let (sender, reciever) = channel::<String>();
    thread::spawn(move || capture_interrupts(sender));

    run_break(args, reciever)
}

fn run_break(cli_args: Cli, receiver: Receiver<String>) -> Result_anyhow<()> {
    let sleep_duration = Duration::new(1, 0);
    let mut break_freq = cli_args.freq.clone();
    let mut break_length = cli_args.len.clone();
    let mut is_break = false;
    let idle_reset_time = cli_args.reset.clone();

    loop {
        match receiver.try_recv() {
            Ok(interrupt) => match interrupt.as_str() {
                "N" | "n" => {
                    is_break = !is_break;
                    break_freq = cli_args.freq.clone();
                    break_length = cli_args.len.clone();
                }
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
            let handle = procspawn::spawn(
                (break_length, sleep_duration),
                |(mut break_length, sleep_duration)| {
                    let (event_loop, event_loop_proxy) = get_event_loop();
                    let app = get_app(event_loop_proxy.clone());

                    let thread_event_loop_proxy = event_loop_proxy.clone();
                    thread::spawn(move || loop {
                        break_length.sub_assign(sleep_duration);
                        if break_length.is_zero() {
                            let _ = thread_event_loop_proxy.send_event(CustomEvent::CLOSE);
                        }
                        thread::sleep(sleep_duration);
                    });
                    run_app(event_loop, app, &event_loop_proxy);
                },
            );

            let _result = handle.join().unwrap();
            break_length = cli_args.len.clone();
            is_break = false;
        } else {
            break_freq.sub_assign(sleep_duration);
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
                continue;
            }

            if let Some(reset) = idle_reset_time {
                let idle_time = get_idle_time();
                if idle_time.ge(&reset) {
                    break_freq = cli_args.freq.clone();
                }
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
