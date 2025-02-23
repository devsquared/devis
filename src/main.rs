use clap::Parser;
use color_eyre::eyre::Result;
use std::{fs::File, io::{self, stdout, BufReader, Write}, process::exit};

mod config;

// TODO: deal with error cases from all run commands here
fn run() -> Result<()> {
    let args = config::CommandLineArgs::parse();

    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    find_matches(&args.pattern, reader, stdout())
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