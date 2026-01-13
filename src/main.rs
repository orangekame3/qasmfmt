use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

use qasmfmt::{FormatConfig, format_with_config};

#[derive(Parser)]
#[command(name = "qasmfmt")]
#[command(author, version, about = "OpenQASM 3.0 formatter", long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    #[arg(short, long)]
    write: bool,

    #[arg(short, long)]
    check: bool,

    #[arg(short, long)]
    diff: bool,

    #[arg(short, long, default_value = "4")]
    indent: usize,

    #[arg(long, default_value = "100")]
    max_width: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = FormatConfig {
        indent_size: cli.indent,
        max_width: cli.max_width,
        ..Default::default()
    };

    if cli.files.is_empty() {
        format_stdin(&config)
    } else {
        let mut has_error = false;
        for file in &cli.files {
            if let Err(e) = format_file(file, &config, cli.write, cli.check, cli.diff) {
                eprintln!("Error processing {}: {}", file.display(), e);
                has_error = true;
            }
        }
        if has_error {
            std::process::exit(1);
        }
        Ok(())
    }
}

fn format_stdin(config: &FormatConfig) -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output = format_with_config(&input, config.clone()).context("Failed to format input")?;

    print!("{}", output);
    Ok(())
}

fn format_file(
    path: &PathBuf,
    config: &FormatConfig,
    write: bool,
    check: bool,
    diff: bool,
) -> Result<()> {
    let input =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let output = format_with_config(&input, config.clone())
        .with_context(|| format!("Failed to format {}", path.display()))?;

    if check {
        if input != output {
            eprintln!("Would reformat: {}", path.display());
            std::process::exit(1);
        }
    } else if diff {
        print_diff(&input, &output, path);
    } else if write {
        fs::write(path, &output).with_context(|| format!("Failed to write {}", path.display()))?;
        eprintln!("Formatted: {}", path.display());
    } else {
        print!("{}", output);
    }

    Ok(())
}

fn print_diff(original: &str, formatted: &str, path: &Path) {
    if original == formatted {
        return;
    }

    println!("--- {}", path.display());
    println!("+++ {}", path.display());

    let orig_lines: Vec<&str> = original.lines().collect();
    let fmt_lines: Vec<&str> = formatted.lines().collect();

    let max_lines = orig_lines.len().max(fmt_lines.len());

    for i in 0..max_lines {
        let orig_line = orig_lines.get(i).copied().unwrap_or("");
        let fmt_line = fmt_lines.get(i).copied().unwrap_or("");

        if orig_line != fmt_line {
            if i < orig_lines.len() {
                println!("-{}: {}", i + 1, orig_line);
            }
            if i < fmt_lines.len() {
                println!("+{}: {}", i + 1, fmt_line);
            }
        }
    }
}
