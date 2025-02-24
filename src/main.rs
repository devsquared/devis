use clap::Parser;
use color_eyre::eyre::{Result};
use config::Commands;
use noteit::create_note;
use std::{io::{self, Write}, process::exit};
use std::result::Result::Ok;

mod config;
mod noteit;

// TODO: deal with error cases from all run commands here
fn run() -> Result<()> {
    let args = config::CommandLineArgs::parse();

    match args.cmd {
        Commands::NoteIt { name, with_toc } => return create_note(name, with_toc),
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

fn find_matches(pattern: &str, reader: impl std::io::BufRead, writer: impl std::io::Write) -> Result<()> {
    let mut wribuffer = io::BufWriter::new(writer);
    for line in reader.lines() {
        let l = line?;
        
        if l.contains(pattern) {
           writeln!(wribuffer, "{}", l)?;
        }
    }
    wribuffer.flush()?;

    Ok(())
}