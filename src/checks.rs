use crate::system::System;
use std::io;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub use macos::CHECKS;

use rayon::iter::IntoParallelRefIterator;
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

#[derive(Debug, thiserror::Error)]
pub enum ChecksError {
    #[error("unknown checks: {}", .0.join(", "))]
    UnknownChecks(Vec<String>),
}

/// enumeration of checks
#[derive(Debug, Clone)]
pub struct Checks(Vec<&'static Check>);

impl Checks {
    pub fn all() -> Self {
        Self(CHECKS.iter().collect())
    }

    pub fn select(selected: &[String]) -> Result<Self, ChecksError> {
        if selected.is_empty() {
            return Ok(Self::all());
        }

        let mut checks = Vec::new();
        let mut unknown = Vec::new();

        for id in selected {
            match CHECKS.iter().find(|check| check.id == id) {
                Some(check) => checks.push(check),
                None => unknown.push(id.clone()),
            }
        }

        if !unknown.is_empty() {
            return Err(ChecksError::UnknownChecks(unknown));
        }

        Ok(Self(checks))
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static Check> + '_ {
        self.0.iter().copied()
    }

    pub fn par_iter(&self) -> rayon::slice::Iter<'_, &'static Check> {
        self.0.par_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
