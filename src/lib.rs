use anyhow::{Context, Error, Result};
use clap::Parser;
use std::time::Duration;

// A break timer tool to help you get up when it's time!
#[derive(Parser, Debug)]
pub struct Cli {
    /// Break frequeny; Duration of time in format: `hh:mm`
    #[arg(value_parser = parse_duration)]
    pub freq: Duration,

    /// Break length; Duration of time in format: `hh:mm`
    #[arg(value_parser = parse_duration)]
    pub len: Duration,

    /// Idle Reset; Duration of time in format: `hh:mm`
    #[arg(value_parser = parse_duration)]
    pub reset: Option<Duration>,
}

fn time_parse_err_msg() -> String {
    "Number must be between 0 and 255".to_string()
}
fn parse_duration(arg: &str) -> Result<Duration> {
    let mut s = arg.split(":");
    if let (Some(hrs), Some(mins)) = (s.next(), s.next()) {
        let (hrs, mins): (u8, u8) = (
            hrs.parse().with_context(time_parse_err_msg)?,
            mins.parse().with_context(time_parse_err_msg)?,
        );

        let time: u64 = (u64::from(hrs) * 60 * 60) + (u64::from(mins) * 60);
        if time.eq(&0) {
            return Err(Error::msg("Duration must be at least 0"));
        }
        Ok(Duration::new(time, 0))
    } else {
        return Err(Error::msg(
            "Please use the required format for the duration",
        ));
    }
}
