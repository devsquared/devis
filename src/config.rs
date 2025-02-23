use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="devis", subcommand_required(true))]
pub struct CommandLineArgs {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    NoteIt{
        #[arg(short = 'n', long="name", required(true))]
        name: String,
        #[arg(short = 't', long="withtoc", required(false))]
        with_toc: bool,
    },
}