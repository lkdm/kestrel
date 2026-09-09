use std::io;

use crate::system::System;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub use macos::CHECKS;

#[cfg(target_os = "windows")]
pub use windows::CHECKS;

#[cfg(target_os = "linux")]
pub use linux::CHECKS;

#[derive(Debug, Clone, Copy)]
pub struct Check {
    /// unique check identifier
    pub id: &'static str,
    /// human-readable description
    pub description: &'static str,
    /// function to run the check
    pub run: CheckFn,
}

/// runs the check
pub type CheckFn = fn(&dyn System) -> CheckResult;

/// answers: did the check pass?
pub type CheckResult = io::Result<bool>;
