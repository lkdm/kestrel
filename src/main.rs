use std::{print, println};

use crate::{checks::CHECKS, system::SystemImpl};

pub mod checks;
pub mod system;

fn main() {
    let system = SystemImpl;
    let checks = CHECKS;
    for check in checks {
        let result = (check.run)(&system);
        println!("{:?}", result)
    }
    println!("done");
}
