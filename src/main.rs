use std::process::exit;
use std::result::Result::Ok;

use clap::Parser;
use color_eyre::eyre::Result;
use config::{Commands, ConfigFile};
use note::create_note;

mod config;
mod note;

// TODO: deal with error cases from all run commands here
fn run() -> Result<()> {
    color_eyre::install()?;

    let config = match ConfigFile::check_file() { // check if config file exists; if so, read the contents to config
        Some(_) => {},
        None => ConfigFile::create_default(),
    };

    let args = config::CommandLineArgs::parse();
    match args.cmd {
        Commands::Note { name, path, with_toc } => return create_note(name, path, with_toc, config.default_note_dir),
    }
}

fn main() {
    match run() {
        Ok(()) => {
            exit(0);
        }
        Err(_) => { // utilize below to pretty print all context and debug info if verbose
            exit(1);
        }
    }
}
