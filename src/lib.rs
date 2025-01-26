use anyhow::{Context, Error, Result};
use clap::Parser;
use std::{num::ParseIntError, time::Duration};

// A break timer tool to help you get up when it's time!
#[derive(Parser, Debug)]
pub struct Cli {
    /// Break frequeny; Time in format: `hh:mm`
    #[arg(value_parser = parse_duration)]
    pub freq: Duration,

    /// Break length; Time in format: `hh:mm`
    #[arg(value_parser = parse_duration)]
    pub len: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration> {
    let mut s = arg.split(":");
    if let (Some(hrs), Some(mins)) = (s.next(), s.next()) {
        let (hrs, mins): (u8, u8) = (hrs.parse()?, mins.parse()?);
        println!("{hrs}, {mins}");
    } else {
        return Err(Error::msg(
            "Please use the required format for the duration",
        ));
    }
    //.map_err(|_| format!("parse error"));

    Ok(Duration::new(5, 0))
}

// Read config
// Save it somewhere
// Use it for later to bootup the process without reading config again
// Use confy
