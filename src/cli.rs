use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::constants::VERSION;
use crate::error::AppError;
use crate::list::ListChecksResult;
use crate::scan::{CheckOutcome, ScanResult};

#[derive(Debug, Parser)]
#[command(name = "kestrel")]
#[command(about = "scans your device to determine your security posture", long_about = None, version=VERSION)]
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
        #[command(subcommand)]
        command: Option<ListCommands>,
        // /// List only check names, separated by spaces for use with `--checks
        // #[arg(long)]
        // names: bool,
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

pub fn print_scan(result: ScanResult, output: OutputFormat) -> Result<(), AppError> {
    match output {
        OutputFormat::Human => {
            print_scan_human(result);
            Ok(())
        }
        OutputFormat::Simple => {
            print_scan_simple(result);
            Ok(())
        }
        OutputFormat::Json => {
            print_scan_json(result)?;
            Ok(())
        }
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

    for check in result.checks {
        match &check.result {
            CheckOutcome::Passed => passed.push(check),
            CheckOutcome::Failed => failed.push(check),
            CheckOutcome::Error => errors.push(check),
        }
    }

    if !passed.is_empty() {
        println!("\n{}", style("PASS").green().bold());

        for check in &passed {
            println!("  {} {}", style("✓").green(), check.message);
        }
    }

    if !failed.is_empty() {
        println!("\n{}", style("FAIL").red().bold());

        for check in &failed {
            if let CheckOutcome::Failed = &check.result {
                println!("  {} {}", style("✗").red(), check.title);
                println!("    → {}", check.message);
            }
        }
    }

    if !errors.is_empty() {
        println!("\n{}", style("ERROR").yellow().bold());

        for check in &errors {
            if let CheckOutcome::Error = &check.result {
                println!(
                    "  {} {}: {}",
                    style("!").yellow(),
                    check.name,
                    check.message
                );
            }
        }
    }
}

pub fn print_scan_simple(result: ScanResult) {
    for check in result.checks {
        match check.result {
            CheckOutcome::Passed => {
                println!("PASS {}", check.name);
            }
            CheckOutcome::Failed { .. } => {
                println!("FAIL {}", check.name);
            }
            CheckOutcome::Error => {
                println!("ERROR {}: {}", check.name, check.title);
            }
        }
    }
}

pub fn print_scan_json(result: ScanResult) -> Result<(), serde_json::Error> {
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}

pub fn print_checks(result: ListChecksResult) {
    let id_width = result
        .checks
        .iter()
        .map(|check| check.id.to_string().len())
        .max()
        .unwrap_or(0);

    let name_width = result
        .checks
        .iter()
        .map(|check| check.name.len())
        .max()
        .unwrap_or(0);

    for check in result.checks {
        println!(
            "{:<id_width$}  {:<name_width$}  {}",
            check.id, check.name, check.title,
        );
    }
}

pub fn print_check_names(result: ListChecksResult) {
    println!(
        "{}",
        result
            .checks
            .iter()
            .map(|check| check.name)
            .collect::<Vec<_>>()
            .join(" ")
    );
}

pub fn print_check_ids(result: ListChecksResult) {
    println!(
        "{}",
        result
            .checks
            .iter()
            .map(|check| check.id.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

pub fn print_error(error: &AppError) {
    eprintln!("{}", style(error).red());
}

#[derive(Debug, Subcommand)]
pub enum ListCommands {
    /// List check names.
    Names,

    /// List check IDs.
    Ids,
}
