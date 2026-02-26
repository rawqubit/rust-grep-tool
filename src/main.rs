use anyhow::Result;
use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The pattern to search for
    pattern: String,

    /// The path to the file or directory to search in
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Case-insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Show line numbers
    #[arg(short, long)]
    line_number: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let pattern = if cli.ignore_case {
        cli.pattern.to_lowercase()
    } else {
        cli.pattern.clone()
    };

    for entry in WalkDir::new(&cli.path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let file = match File::open(path) {
                Ok(f) => f,
                Err(_) => continue, // Skip files that can't be opened
            };
            let reader = BufReader::new(file);

            for (index, line) in reader.lines().enumerate() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => break, // Skip lines that can't be read (e.g., binary files)
                };
                let search_line = if cli.ignore_case {
                    line.to_lowercase()
                } else {
                    line.clone()
                };

                if search_line.contains(&pattern) {
                    let display_path = path.display().to_string().bright_blue();
                    let line_num = if cli.line_number {
                        format!("{}:", index + 1).yellow()
                    } else {
                        "".clear()
                    };

                    // Highlight the pattern in the line
                    let highlighted_line = highlight_pattern(&line, &cli.pattern, cli.ignore_case);
                    println!("{} {}{}", display_path, line_num, highlighted_line);
                }
            }
        }
    }

    Ok(())
}

fn highlight_pattern(line: &str, pattern: &str, ignore_case: bool) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    let search_line = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };
    let search_pattern = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    for (start, _) in search_line.match_indices(&search_pattern) {
        result.push_str(&line[last_end..start]);
        result.push_str(&line[start..start + pattern.len()].red().bold().to_string());
        last_end = start + pattern.len();
    }
    result.push_str(&line[last_end..]);
    result
}
