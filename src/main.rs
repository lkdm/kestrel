use crate::checks::Checks;
use crate::cli::{Cli, Commands, ListCommands, ProgressMode};
use crate::error::AppError;
use crate::list::list_checks;
use crate::scan::ScanContext;
use crate::system::SystemImpl;
use clap::Parser;

pub mod checks;
pub mod cli;
pub mod constants;
pub mod error;
pub mod list;
pub mod scan;
pub mod system;
pub mod tracing;

// ## Road to release
//
// Correctness
// - Co-located unit tests with mock
// - Tests for every existing check
// - Test command error classification
// - Test check selection by name and ID
// - Test duplicate check detection
// - Test JSON output/schema
//
// Checks
// - 20-30 solid macOS checks
// - Check metadata/references/guide
// - Consistent check behavior and error handling
//
// CLI
// - Useful exit codes
// - Stable machine-readable output
// - Good --help output
// - Clear error messages
//
// Project
// - CI
// - Licence
// - Basic contribution guide
// - README with examples
//
// Distribution
// - Release binaries
// - Versioning / release process

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
        Commands::ListChecks { command } => {
            let result = list_checks();

            match command {
                None => cli::print_checks(result),
                Some(ListCommands::Names) => cli::print_check_names(result),
                Some(ListCommands::Ids) => cli::print_check_ids(result),
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

            cli::print_scan(result, output)?;
        }

        Commands::Config { command } => {}
    }

    Ok(())
}
