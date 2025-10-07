// This is the most popular library for CLI parsing
// https://docs.rs/clap/latest/clap/
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
}
