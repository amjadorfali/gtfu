fn main() {}
// fn main() -> std::io::Result<()> {
//     use clap_mangen;
//
//     #[path = "src/lib.rs"]
//     mod cli;
//     let out_dir = std::path::PathBuf::from(
//         std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?,
//     );
//     let cmd = cli::Cli::command();
//
//     let man = clap_mangen::Man::new(cmd);
//     let mut buffer: Vec<u8> = Default::default();
//     man.render(&mut buffer)?;
//
//     std::fs::write(out_dir.join("head.1"), buffer)?;
//
//     Ok(())
// }
