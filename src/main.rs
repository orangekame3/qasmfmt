use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;
use walkdir::WalkDir;

use qasmfmt::{FormatConfig, format_with_config};

#[derive(Parser)]
#[command(name = "qasmfmt")]
#[command(author, version, about = "OpenQASM 3.0 formatter", long_about = None)]
struct Cli {
    /// Files or directories to format. Use '-' for stdin.
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,

    /// Write formatted output back to files (in-place)
    #[arg(short, long)]
    write: bool,

    /// Check if files are formatted (exit 1 if not)
    #[arg(long)]
    check: bool,

    /// Show unified diff of formatting changes (exit 1 if diff exists)
    #[arg(long)]
    diff: bool,

    /// Indentation size in spaces (default: 4)
    #[arg(short, long)]
    indent: Option<usize>,

    /// Maximum line width (default: 100)
    #[arg(long)]
    max_width: Option<usize>,

    /// Virtual filename for stdin input (used for config lookup and error messages)
    #[arg(long, value_name = "PATH")]
    stdin_filename: Option<PathBuf>,

    /// Path to configuration file
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Disable automatic configuration file discovery
    #[arg(long)]
    no_config: bool,
}

/// Represents the formatting mode
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Print,
    Write,
    Check,
    Diff,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Validate mutually exclusive modes
    let mode = validate_mode(&cli)?;

    // Check stdin + write combination
    let has_stdin = cli.paths.iter().any(|p| p.as_os_str() == "-")
        || (cli.paths.is_empty() && !io::stdin().is_terminal());

    if has_stdin && mode == Mode::Write {
        eprintln!("error: cannot use --write with stdin input");
        std::process::exit(2);
    }

    // Load configuration
    let mut config = load_config(&cli)?;

    // Apply CLI overrides (only if explicitly provided)
    if let Some(indent) = cli.indent {
        config.indent_size = indent;
    }
    if let Some(max_width) = cli.max_width {
        config.max_width = max_width;
    }

    // Determine input sources
    if cli.paths.is_empty() {
        // No paths provided: read from stdin if it's a pipe
        if !io::stdin().is_terminal() {
            let has_diff = format_stdin(&config, cli.stdin_filename.as_deref(), mode)?;
            if has_diff && (mode == Mode::Check || mode == Mode::Diff) {
                std::process::exit(1);
            }
            Ok(())
        } else {
            bail!("no input files provided");
        }
    } else {
        let mut has_error = false;
        let mut has_diff = false;

        for path in &cli.paths {
            if path.as_os_str() == "-" {
                // Explicit stdin
                match format_stdin(&config, cli.stdin_filename.as_deref(), mode) {
                    Ok(diff) => has_diff |= diff,
                    Err(e) => {
                        eprintln!("Error processing stdin: {e}");
                        has_error = true;
                    }
                }
            } else if path.is_dir() {
                // Directory: recursively process .qasm files
                for entry in WalkDir::new(path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.file_type().is_file()
                            && e.path().extension().is_some_and(|ext| ext == "qasm")
                    })
                {
                    match format_file(entry.path(), &config, mode) {
                        Ok(diff) => has_diff |= diff,
                        Err(e) => {
                            eprintln!("Error processing {}: {e}", entry.path().display());
                            has_error = true;
                        }
                    }
                }
            } else {
                // Regular file
                match format_file(path, &config, mode) {
                    Ok(diff) => has_diff |= diff,
                    Err(e) => {
                        eprintln!("Error processing {}: {e}", path.display());
                        has_error = true;
                    }
                }
            }
        }

        if has_error {
            std::process::exit(1);
        }

        // Exit 1 if diff/check mode found differences
        if has_diff && (mode == Mode::Check || mode == Mode::Diff) {
            std::process::exit(1);
        }

        Ok(())
    }
}

/// Validate that only one mode is specified
fn validate_mode(cli: &Cli) -> Result<Mode> {
    let mode_count = [cli.write, cli.check, cli.diff]
        .iter()
        .filter(|&&b| b)
        .count();

    if mode_count > 1 {
        eprintln!("error: --write, --check, and --diff are mutually exclusive");
        std::process::exit(2);
    }

    Ok(if cli.write {
        Mode::Write
    } else if cli.check {
        Mode::Check
    } else if cli.diff {
        Mode::Diff
    } else {
        Mode::Print
    })
}

/// Load configuration from file
fn load_config(cli: &Cli) -> Result<FormatConfig> {
    if cli.no_config {
        return Ok(FormatConfig::default());
    }

    if let Some(config_path) = &cli.config {
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        return FormatConfig::from_toml(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()));
    }

    // Auto-discover config file
    let search_start = if let Some(stdin_filename) = &cli.stdin_filename {
        stdin_filename
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    } else if let Some(first_path) = cli.paths.first() {
        if first_path.as_os_str() == "-" {
            PathBuf::from(".")
        } else if first_path.is_dir() {
            first_path.clone()
        } else {
            first_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        }
    } else {
        PathBuf::from(".")
    };

    if let Some(config) = find_config_file(&search_start)? {
        return Ok(config);
    }

    Ok(FormatConfig::default())
}

/// Search for qasmfmt.toml from the given directory upward
fn find_config_file(start: &Path) -> Result<Option<FormatConfig>> {
    let start = if start.is_absolute() {
        start.to_path_buf()
    } else {
        std::env::current_dir()?.join(start)
    };

    let mut current = start.as_path();

    loop {
        let config_path = current.join("qasmfmt.toml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).with_context(|| {
                format!("Failed to read config file: {}", config_path.display())
            })?;
            let config = FormatConfig::from_toml(&content).with_context(|| {
                format!("Failed to parse config file: {}", config_path.display())
            })?;
            return Ok(Some(config));
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    Ok(None)
}

/// Format input from stdin
fn format_stdin(config: &FormatConfig, filename: Option<&Path>, mode: Mode) -> Result<bool> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output = format_with_config(&input, config.clone()).context("Failed to format input")?;

    let display_path = filename.unwrap_or(Path::new("<stdin>"));

    match mode {
        Mode::Print | Mode::Write => {
            print!("{}", output);
            Ok(false)
        }
        Mode::Check => {
            if input != output {
                eprintln!("Would reformat: {}", display_path.display());
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Mode::Diff => {
            if input != output {
                print_diff(&input, &output, display_path);
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

/// Format a file
fn format_file(path: &Path, config: &FormatConfig, mode: Mode) -> Result<bool> {
    let input =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let output = format_with_config(&input, config.clone())
        .with_context(|| format!("Failed to format {}", path.display()))?;

    match mode {
        Mode::Print => {
            print!("{}", output);
            Ok(false)
        }
        Mode::Write => {
            if input != output {
                fs::write(path, &output)
                    .with_context(|| format!("Failed to write {}", path.display()))?;
                eprintln!("Formatted: {}", path.display());
            }
            Ok(false)
        }
        Mode::Check => {
            if input != output {
                eprintln!("Would reformat: {}", path.display());
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Mode::Diff => {
            if input != output {
                print_diff(&input, &output, path);
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

/// Print unified diff between original and formatted content
fn print_diff(original: &str, formatted: &str, path: &Path) {
    let orig_lines: Vec<&str> = original.lines().collect();
    let fmt_lines: Vec<&str> = formatted.lines().collect();

    let max_lines = orig_lines.len().max(fmt_lines.len());

    // Collect diff lines first
    let mut diff_lines = Vec::new();
    for i in 0..max_lines {
        let orig_line = orig_lines.get(i).copied().unwrap_or("");
        let fmt_line = fmt_lines.get(i).copied().unwrap_or("");

        if orig_line != fmt_line {
            if i < orig_lines.len() {
                diff_lines.push(format!("-{}: {}", i + 1, orig_line));
            }
            if i < fmt_lines.len() {
                diff_lines.push(format!("+{}: {}", i + 1, fmt_line));
            }
        }
    }

    // Only print if there are actual line differences
    if !diff_lines.is_empty() {
        println!("--- {}", path.display());
        println!("+++ {}", path.display());
        for line in diff_lines {
            println!("{}", line);
        }
    }
}
