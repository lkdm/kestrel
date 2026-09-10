use crate::checks::Checks;
use crate::error::AppError;
use crate::list::list_checks;
use crate::scan::ScanContext;
use crate::{checks::CHECKS, system::SystemImpl};
use crate::{
    checks::Check,
    cli::{Cli, Commands},
};
use clap::{Parser, Subcommand, ValueEnum};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{print, println};

pub mod checks;
pub mod cli;
pub mod error;
pub mod list;
pub mod scan;
pub mod system;

fn main() {
    if let Err(error) = run() {
        cli::print_error(&error);
        std::process::exit(2);
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let system = SystemImpl;

    match cli.command {
        Commands::ListChecks => {
            let result = list_checks();
            cli::print_checks(result);
        }

        Commands::Scan {
            checks,
            output,
            no_progress,
        } => {
            let checks = Checks::select(&checks)?;
            let context = ScanContext::new(&checks);
            let progress = cli::progress(checks.len(), !no_progress);

            let result = scan::scan(&context, &system, || {
                progress.inc(1);
            });

            progress.finish_and_clear();

            cli::print_scan(result, output);
        }
    }

    Ok(())
}

// fn main() {
//     let system = SystemImpl;
//     let checks = CHECKS;

//     let progress = ProgressBar::new(checks::CHECKS.len() as u64);

//     progress.set_style(
//         ProgressStyle::with_template(
//             "Kestrel is running {spinner:.green.bold}\n\
//              \n\
//              \tElapsed:  {elapsed_precise}\n\
//              \tProgress: {pos}/{len} checks",
//         )
//         .unwrap(),
//     );

//     progress.enable_steady_tick(std::time::Duration::from_millis(100));

//     let results: Vec<_> = checks::CHECKS
//         .par_iter()
//         .map(|check| {
//             let result = (check.run)(&system);
//             progress.inc(1);
//             (check, result)
//         })
//         .collect();

//     progress.finish_and_clear();

//     let mut passed = Vec::new();
//     let mut failed = Vec::new();
//     let mut errors = Vec::new();

//     for (check, result) in results {
//         match result {
//             Ok(true) => passed.push(check),
//             Ok(false) => failed.push(check),
//             Err(error) => errors.push((check, error)),
//         }
//     }

//     let has_failures = !failed.is_empty() || !errors.is_empty();

//     if !passed.is_empty() {
//         println!("\n{}", style("PASS").green().bold());

//         for check in &passed {
//             println!("  {} {}", style("✓").green(), check.description);
//         }
//     }

//     if !failed.is_empty() {
//         println!("\n{}", style("FAIL").red().bold());

//         for check in &failed {
//             println!("  {} {}", style("✗").red(), check.description);
//         }
//     }

//     if !errors.is_empty() {
//         println!("\n{}", style("ERROR").yellow().bold());

//         for (check, error) in &errors {
//             println!("  {} {}: {}", style("!").yellow(), check.description, error);
//         }
//     }

//     std::process::exit(if has_failures { 1 } else { 0 });
// }
