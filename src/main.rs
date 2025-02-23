use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::{fs::File, io::{self, stdout, BufReader, Write}};

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let reader = BufReader::new(file);

    find_matches(&args.pattern, reader, stdout())
}

fn find_matches(pattern: &str, reader: impl std::io::BufRead, writer: impl std::io::Write) -> Result<(), anyhow::Error> {
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

#[cfg(test)]
mod tests {
    use std::io::{stdin, Read};

    use super::*;

    // #[test]
    // fn test_find_matches() {
    //     let mut actual = Vec::new();
    //     let mut reader = BufReader::new(stdin());
    //     stdin.write_all("pattern yeah\nsome more text");
    //     find_matches("pattern", reader, &mut actual);
    //     assert_eq!(actual, b"pattern yeah\n")
    // }
}