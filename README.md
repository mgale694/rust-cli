# rust-cli

A quick Rust CLI example following the [Rust CLI book](https://rust-cli.github.io/book/index.html).

This project implements `grrs` (grep in rust), a simple command-line tool that searches for a pattern in a file and displays the lines that contain it.

## Building

```bash
cargo build
```

## Usage

```bash
cargo run -- <pattern> <file>
```

Or after building:

```bash
./target/debug/grrs <pattern> <file>
```

### Examples

Search for the word "test" in a file:
```bash
cargo run -- test myfile.txt
```

Get help:
```bash
cargo run -- --help
```

## Features

- Simple pattern matching in text files
- Clear error messages with context
- Command-line argument parsing with `clap`
- Proper error handling with `anyhow`