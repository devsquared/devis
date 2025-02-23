use clap::Parser;

#[derive(Parser)]
pub struct CommandLineArgs {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    pub path: std::path::PathBuf,
}