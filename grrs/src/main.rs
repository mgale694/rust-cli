// This is the most popular library for CLI parsing
// https://docs.rs/clap/latest/clap/
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // The pattern to look for
    #[arg(short = 'p', long)]
    pattern: String,

    // The path to the file to read
    #[arg(short = 'f', long)]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Open the file in read-only mode (ignoring errors).
    let f = File::open(&args.path)?;
    // Create a buffered reader on top of that file
    let reader = BufReader::new(f);

    for line in reader.lines() {
        // Ignore errors on lines
        let line = line?;
        // Check if the line contains the pattern
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
