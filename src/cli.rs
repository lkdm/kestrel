use std::io;

use clap::{Parser, Subcommand, ValueEnum};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::checks::{CHECKS, Check};

use crate::checks::Checks;
use crate::error::AppError;
use crate::list::ListChecksResult;
use crate::scan::{ScanContext, ScanResult};

#[derive(Debug, Parser)]
#[command(name = "kestrel")]
#[command(about = "scans your device to determine your security posture", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ListChecks,

    Scan {
        #[arg(long = "checks", value_name = "CHECKS", num_args = 1..)]
        checks: Vec<String>,

        #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
        output: OutputFormat,
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

pub fn progress(len: usize) -> ProgressBar {
    let progress = ProgressBar::new(len as u64);

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

pub fn print_error(error: &AppError) {
    eprintln!("{}", style(error).red());
}
