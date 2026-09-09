use std::{print, println};

use crate::{checks::CHECKS, system::SystemImpl};

pub mod checks;
pub mod system;

fn main() {
    let system = SystemImpl;
    let checks = CHECKS;
    for check in checks {
        match (check.run)(&system) {
            Ok(true) => println!("[PASS] {}", check.description),
            Ok(false) => println!("[FAIL] {}", check.description),
            Err(error) => println!("[ERROR] {}: {}", check.description, error),
        }
    }
    println!("done");
}
