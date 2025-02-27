use std::path::PathBuf;
use std::env::current_exe;

use clap::{Parser, Subcommand};
use color_eyre::eyre::{self, Result};
use dirs::config_dir;
use serde::Deserialize;

static CONFIG_FILE_NAME: &str = "devis.toml";

#[derive(Parser)]
#[command(name="devis", subcommand_required(true))]
pub struct CommandLineArgs {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Note{
        #[arg(short='n', long="name", required(true))]
        name: String,
        #[arg(short='p', long="path", required(false))] // add extra help stuff to note that it will go in default note directory if none provided
        path: PathBuf,
        #[arg(short='t', long="withtoc", required(false))]
        with_toc: bool,
    },
}

// TODO: need to add config file reader and creator here.
//  known needed fields:
//   - default_note_dir
#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
/// Configuration file
pub struct ConfigFile {
    default_note_dir: Option<PathBuf>,
}

impl ConfigFile {
    fn check_file() -> Result<Option<PathBuf>> {
        // check for existence at config dir and local to executable
        

        Ok(None)
    }
}