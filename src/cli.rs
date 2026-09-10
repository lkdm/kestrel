use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::AppError;
use crate::list::ListChecksResult;
use crate::scan::ScanResult;

#[derive(Debug, Parser)]
#[command(name = "kestrel")]
#[command(about = "scans your device to determine your security posture", long_about = None, version)]
pub struct Cli {
    // /// Path to the configuration file
    // /// CLI options are preferenced over config file
    // #[arg(long, env = "KESTREL_CONFIG")]
    // pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// lists checks available for your device
    #[command(name = "list")]
    ListChecks {
        /// List only check names, separated by spaces for use with `--checks
        #[arg(long)]
        names: bool,
    },

    /// scan your device
    Scan {
        /// checks to run; defaults to all checks
        #[arg(long = "checks", value_name = "CHECK", num_args = 1..)]
        checks: Vec<String>,

        /// output format for the result
        #[arg(
            long = "format",
            alias = "output",
            value_enum,
            default_value_t = OutputFormat::Human
        )]
        output: OutputFormat,

        /// how progress is shown
        #[arg(
            long,
            value_enum,
            default_value_t = ProgressMode::Tui
        )]
        progress: ProgressMode,

        /// disable parallelisation
        #[arg(long)]
        no_parallel: bool,
        // /// if a previous scan failed, this will retry the failure or error checks
        // #[arg(long, alias = "continue")]
        // resume: bool,

        // /// if a previous scan failed, this will assist the user with remediatory steps
        // #[arg(long, alias = "remediate")]
        // remediate: bool,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ProgressMode {
    /// progress is shown as an animation
    Tui,
    /// progress is shown as a log
    Logs,
    /// progress is not shown
    None,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Print the path Kestrel uses for its configuration
    Path,

    /// Create a default configuration file
    Init {
        /// Path to write the configuration file
        #[arg(long)]
        path: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Human,
    Simple,
    Json,
}

pub fn print_scan(result: ScanResult, format: OutputFormat) {
    match format {
        OutputFormat::Human => print_scan_human(result),
        OutputFormat::Simple => print_scan_simple(result),
        OutputFormat::Json => print_scan_json(result),
    }
}

pub fn progress(len: usize, mode: ProgressMode) -> ProgressBar {
    let progress = ProgressBar::new(len as u64);

    if !matches!(mode, ProgressMode::Tui) {
        progress.set_draw_target(indicatif::ProgressDrawTarget::hidden());
        return progress;
    }

    progress.set_style(
        ProgressStyle::with_template(
            "Kestrel is running {spinner:.green.bold}\n\
             \n\
             \tElapsed:  {elapsed_precise}\n\
             \tProgress: {pos}/{len} checks",
        )
        .unwrap(),
    );

    progress.enable_steady_tick(std::time::Duration::from_millis(100));

    progress
}

pub fn print_scan_human(result: ScanResult) {
    let mut passed = Vec::new();
    let mut failed = Vec::new();
    let mut errors = Vec::new();

    for (check, result) in result.results {
        match result {
            Ok(true) => passed.push(check),
            Ok(false) => failed.push(check),
            Err(error) => errors.push((check, error)),
        }
    }

    if !passed.is_empty() {
        println!("\n{}", style("PASS").green().bold());

        for check in &passed {
            println!("  {} {}", style("✓").green(), check.description);
        }
    }

    if !failed.is_empty() {
        println!("\n{}", style("FAIL").red().bold());

        for check in &failed {
            println!("  {} {}", style("✗").red(), check.description);
        }
    }

    if !errors.is_empty() {
        println!("\n{}", style("ERROR").yellow().bold());

        for (check, error) in &errors {
            println!("  {} {}: {}", style("!").yellow(), check.description, error);
        }
    }
}

pub fn print_scan_simple(result: ScanResult) {
    for (check, result) in result.results {
        match result {
            Ok(true) => println!("PASS {}", check.id),
            Ok(false) => println!("FAIL {}", check.id),
            Err(error) => println!("ERROR {}: {}", check.id, error),
        }
    }
}

pub fn print_scan_json(result: ScanResult) {
    todo!()
}

pub fn print_checks(result: ListChecksResult) {
    for check in result.checks {
        println!("{}: {}", check.id, check.description);
    }
}

pub fn print_check_names(result: ListChecksResult) {
    println!(
        "{}",
        result
            .checks
            .iter()
            .map(|check| check.id)
            .collect::<Vec<_>>()
            .join(" ")
    );
}

pub fn print_error(error: &AppError) {
    eprintln!("{}", style(error).red());
}
