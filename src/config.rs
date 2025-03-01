use std::fs;
use std::path::{PathBuf};
use std::env::current_exe;
use std::result::Result::Ok;

use clap::{Parser, Subcommand};
use color_eyre::eyre::{eyre, Result};
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
        path: Option<PathBuf>,
        #[arg(short='t', long="withtoc", required(false))]
        with_toc: bool,
    },
}

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
/// Configuration file
pub struct ConfigFile {
   pub default_note_dir: Option<PathBuf>,
}

impl ConfigFile {
    pub fn load(config_path: PathBuf) -> Result<ConfigFile> {
        let contents = match fs::read_to_string(config_path) {
            Ok(c) => Ok(c),
            Err(_) => eyre!("unable to read config file"),
        };

        
    }

    pub fn check_file() -> Option<PathBuf> {
        // check for existence at config dir and local to executable
        let mut found: Option<PathBuf> = None;
        match config_dir() { // check first in config dir
            Some(p) => {
                let devis_config_path = p.join("devis").join("devis.toml");
                if devis_config_path.exists() {
                    found = Some(devis_config_path)
                }
            },
            _ => found = None,
        };

        match current_exe() { // check location of current executable
            Ok(p) => {
                let devis_config_path = p.join("devis.toml");
                if devis_config_path.exists() {
                    found = Some(devis_config_path)
                }
            },
            Err(_) => found = None,
        };

        found
    }

    pub fn create_default() -> Self {
        // write a default config file and return it

        ConfigFile{
            default_note_dir: None
        }
    }
}