use std::io::Write;

use clap::CommandFactory;

#[path = "src/cli_parser.rs"]
mod cli_parser;

use crate::cli_parser::Cli;
fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?,
    );
    let cmd = Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();

    man.render(&mut buffer)?;
    buffer.write(b"ezz ggggggg ff").unwrap();

    std::fs::write(out_dir.join("head.1"), buffer)?;

    Ok(())
}
