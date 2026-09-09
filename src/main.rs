use crate::{checks::CHECKS, system::SystemImpl};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{print, println};

pub mod checks;
pub mod system;

fn main() {
    let system = SystemImpl;
    let checks = CHECKS;

    let progress = ProgressBar::new(checks::CHECKS.len() as u64);

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

    let results: Vec<_> = checks::CHECKS
        .par_iter()
        .map(|check| {
            let result = (check.run)(&system);
            progress.inc(1);
            (check, result)
        })
        .collect();

    progress.finish_and_clear();

    let mut passed = Vec::new();
    let mut failed = Vec::new();
    let mut errors = Vec::new();

    for (check, result) in results {
        match result {
            Ok(true) => passed.push(check),
            Ok(false) => failed.push(check),
            Err(error) => errors.push((check, error)),
        }
    }

    if !passed.is_empty() {
        println!("\n{}", style("PASS").green().bold());

        for check in passed {
            println!("  {} {}", style("✓").green(), check.description);
        }
    }

    if !failed.is_empty() {
        println!("\n{}", style("FAIL").red().bold());

        for check in failed {
            println!("  {} {}", style("✗").red(), check.description);
        }
    }

    if !errors.is_empty() {
        println!("\n{}", style("ERROR").yellow().bold());

        for (check, error) in errors {
            println!("  {} {}: {}", style("!").yellow(), check.description, error);
        }
    }
}
