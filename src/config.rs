use std::path::PathBuf;

use clap::{Parser, Subcommand};

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