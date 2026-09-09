use crate::{checks::CHECKS, system::SystemImpl};
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

    for (check, result) in results {
        match result {
            Ok(true) => println!("[PASS] {}", check.description),
            Ok(false) => println!("[FAIL] {}", check.description),
            Err(error) => println!("[ERROR] {}: {}", check.description, error),
        }
    }
    println!("done");
}
