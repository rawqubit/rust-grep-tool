# Rust Grep Tool 🔍

A fast and simple CLI file search tool built with Rust. It allows you to search for patterns in files and directories, with support for case-insensitive search and line numbering.

## Features

- **Fast Search:** Efficiently searches through files and directories.
- **Pattern Highlighting:** Highlights the matched pattern in the output.
- **Case-Insensitive Search:** Optional case-insensitive matching.
- **Line Numbering:** Optional display of line numbers for matches.
- **Recursive Search:** Automatically searches through subdirectories.

## Installation

To build the tool from source, you need to have Rust and Cargo installed.

```bash
git clone https://github.com/rawqubit/rust-grep-tool.git
cd rust-grep-tool
cargo build --release
```

The binary will be available at `target/release/rust-grep-tool`.

## Usage

```bash
rust-grep-tool [OPTIONS] <PATTERN> [PATH]
```

### Arguments

- `<PATTERN>`: The pattern to search for.
- `[PATH]`: The path to the file or directory to search in (default: `.`).

### Options

- `-i, --ignore-case`: Perform case-insensitive search.
- `-l, --line-number`: Show line numbers for matches.
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

## Examples

Search for "main" in the current directory:
```bash
rust-grep-tool "main"
```

Search for "pattern" in a specific file with line numbers:
```bash
rust-grep-tool -l "pattern" src/main.rs
```

Case-insensitive search for "rust" in the `src` directory:
```bash
rust-grep-tool -i "rust" src
```


