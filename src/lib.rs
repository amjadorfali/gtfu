use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,

    /// The path to the file to read
    #[arg(default_value = "")]
    pub path: std::path::PathBuf,
}

// Read config
// Save it somewhere
// Use it for later to bootup the process without reading config again
// Use confy
