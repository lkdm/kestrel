use crate::{checks::CHECKS, system::SystemImpl};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{print, println};

pub mod checks;
pub mod system;

fn main() {
    let system = SystemImpl;
    let checks = CHECKS;

    let spinner = ProgressBar::new_spinner();
    spinner
        .set_style(ProgressStyle::with_template("{spinner} Running security checks...").unwrap());
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let results: Vec<_> = checks
        .par_iter()
        .map(|check| {
            let result = (check.run)(&system);
            (check, result)
        })
        .collect();

    spinner.finish_and_clear();

    for (check, result) in results {
        match result {
            Ok(true) => println!("[PASS] {}", check.description),
            Ok(false) => println!("[FAIL] {}", check.description),
            Err(error) => println!("[ERROR] {}: {}", check.description, error),
        }
    }
    println!("done");
}
