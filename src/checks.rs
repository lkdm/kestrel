use crate::system::{System, SystemError};
use std::{collections::HashSet, io};

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

pub mod shared;

#[cfg(target_os = "macos")]
pub use macos::CHECKS;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;
#[cfg(target_os = "windows")]
pub use windows::CHECKS;

#[cfg(target_os = "linux")]
pub use linux::CHECKS;

#[derive(Debug, Error)]
pub enum CheckError {
    #[error("system error: {0}")]
    System(#[from] SystemError),
}

pub type Result<T> = std::result::Result<T, CheckError>;

#[derive(Debug, Clone, Copy)]
pub struct Check {
    pub id: CheckId,
    /// unique check identifier
    pub name: &'static str,
    /// human-readable description
    pub description: &'static str,
    /// reccomendation for remediation
    pub recommendation: &'static str,
    /// function to run the check
    pub run: CheckFn,
}

/// runs the check
pub type CheckFn = fn(&dyn System) -> CheckReturnedResult;

pub type CheckReturnedResult = Result<bool>;

#[derive(Debug, thiserror::Error)]
pub enum ChecksError {
    #[error("unknown checks: {}", .0.join(", "))]
    UnknownChecks(Vec<String>),

    #[error("duplicate checks:\n  {}", .0.join("\n  "))]
    DuplicateChecks(Vec<String>),
}

/// enumeration of checks
#[derive(Debug, Clone)]
pub struct Checks(Vec<&'static Check>);

impl Checks {
    pub fn all() -> Self {
        Self(CHECKS.iter().collect())
    }

    pub fn select(selected: &[String]) -> std::result::Result<Self, ChecksError> {
        if selected.is_empty() {
            return Ok(Self::all());
        }

        let mut checks = Vec::new();
        let mut selected_ids = HashSet::new();
        let mut unknown = Vec::new();
        let mut duplicate_ids = HashSet::new();
        let mut duplicates = Vec::new();

        for value in selected {
            let check = value
                .parse::<CheckId>()
                .ok()
                .and_then(|id| CHECKS.iter().find(|check| check.id == id))
                .or_else(|| CHECKS.iter().find(|check| check.name == value));

            match check {
                Some(check) if !selected_ids.insert(check.id) => {
                    if duplicate_ids.insert(check.id) {
                        duplicates.push(format!("{} ({})", check.name, check.id));
                    }
                }
                Some(check) => checks.push(check),
                None => unknown.push(value.clone()),
            }
        }

        if !unknown.is_empty() {
            return Err(ChecksError::UnknownChecks(unknown));
        }

        if !duplicates.is_empty() {
            return Err(ChecksError::DuplicateChecks(duplicates));
        }

        Ok(Self(checks))
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static Check> + '_ {
        self.0.iter().copied()
    }

    pub fn par_iter(&self) -> impl ParallelIterator<Item = &'static Check> + '_ {
        self.0.par_iter().copied()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckId(Uuid);

impl CheckId {
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for CheckId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> std::result::Result<CheckId, uuid::Error> {
        Ok(Self(Uuid::parse_str(value)?))
    }
}

impl std::fmt::Display for CheckId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[macro_export]
macro_rules! check_id {
    ($uuid:literal) => {
        $crate::checks::CheckId::new(uuid::uuid!($uuid))
    };
}
