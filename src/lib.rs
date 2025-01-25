use anyhow::{Context, Result};
use clap::Parser;
use std::{num::ParseIntError, time::Duration};

// Search for a pattern in a file and display the lines that contain it.
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
    arg.parse::<u8>()?;
    //.map_err(|_| format!("parse error"));

    Ok(Duration::new(5, 0))
}

// Read config
// Save it somewhere
// Use it for later to bootup the process without reading config again
// Use confy
