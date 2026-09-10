use crate::checks::Checks;
use crate::cli::{Cli, Commands, ProgressMode};
use crate::error::AppError;
use crate::list::list_checks;
use crate::scan::ScanContext;
use crate::system::SystemImpl;
use clap::Parser;

pub mod checks;
pub mod cli;
pub mod error;
pub mod list;
pub mod scan;
pub mod system;
pub mod tracing;

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
        Commands::ListChecks { names } => {
            let result = list_checks();

            if names {
                cli::print_check_names(result);
            } else {
                cli::print_checks(result);
            }
        }

        Commands::Scan {
            checks,
            output,
            progress,
            no_parallel,
        } => {
            if matches!(progress, ProgressMode::Logs) {
                tracing::init_terminal();
            }

            let checks = Checks::select(&checks)?;
            let context = ScanContext::new(&checks, !no_parallel);
            let progress = cli::progress(checks.len(), progress);

            let result = scan::scan(&context, &system, || {
                progress.inc(1);
            });

            progress.finish_and_clear();

            cli::print_scan(result, output);
        }

        Commands::Config { command } => {}
    }

    Ok(())
}
