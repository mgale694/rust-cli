# grrs - A Fast File Search Tool 🔍

A simple and efficient command-line utility written in Rust for searching text patterns within files. Think of it as a lightweight `grep` alternative built for learning and demonstration purposes.

## Features

- **Fast text search**: Efficiently searches for patterns in text files
- **Simple CLI interface**: Easy-to-use command-line arguments
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Memory efficient**: Uses buffered reading for large files
- **Rust-powered**: Built with Rust for performance and safety

## Installation

### From Source

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)

2. Clone this repository:

   ```bash
   git clone https://github.com/mgale694/grrs.git
   cd grrs
   ```

3. Build and install:
   ```bash
   cargo build --release
   cargo install --path .
   ```

## Usage

```bash
grrs [OPTIONS] --pattern <PATTERN> --file <FILE>
```

### Arguments

- `-p, --pattern <PATTERN>`: The text pattern to search for
- `-f, --file <FILE>`: The path to the file to search in

### Examples

Search for "error" in a log file:

```bash
grrs --pattern "error" --file /var/log/app.log
```

Using short flags:

```bash
grrs -p "TODO" -f src/main.rs
```

Search for a phrase with spaces:

```bash
grrs --pattern "function main" --file src/lib.rs
```

## Development

### Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- --pattern "search_term" --file "path/to/file.txt"
```

## Project Structure

```
grrs/
├── src/
│   ├── main.rs      # CLI application entry point
│   └── lib.rs       # Core search functionality
├── tests/
│   └── test_matching.rs  # Integration tests
├── Cargo.toml       # Project configuration
└── README.md        # This file
```

## Dependencies

- **clap**: Command-line argument parsing
- **tempfile**: Testing utilities (dev dependency)
- **assert_cmd**: Command testing (dev dependency)
- **predicates**: Test assertions (dev dependency)
- **assert_fs**: Filesystem testing (dev dependency)

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Author

Matt Gale (mgale694@gmail.com)

---

_This project was created as a learning exercise following the Rust CLI tutorial. It demonstrates fundamental Rust concepts including CLI parsing, file I/O, error handling, and testing._
