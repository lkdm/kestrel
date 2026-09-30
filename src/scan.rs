use std::time::Instant;

use chrono::{DateTime, Utc};
use rayon::iter::ParallelIterator;
use serde::Serialize;

use crate::{
    checks::{Check, CheckReturnedResult, Checks},
    constants::VERSION,
    system::System,
};

/// context for starting a scan
#[derive(Debug, Clone)]
pub struct ScanContext {
    /// subset of checks to be scanned
    pub checks: Checks,
    pub parallel: bool,
}

impl ScanContext {
    pub fn new(checks: &Checks, parallel: bool) -> Self {
        Self {
            checks: checks.clone(),
            parallel,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub id: String,
    pub name: &'static str,
    pub title: &'static str,
    pub message: String,
    pub result: CheckOutcome,
}

#[derive(Debug, Serialize)]
pub enum CheckOutcome {
    Passed,
    Failed,
    Error,
}

impl CheckResult {
    pub fn new(check: &Check, result: CheckReturnedResult) -> Self {
        let (result, message) = match result {
            Ok(true) => (CheckOutcome::Passed, check.passed_message.to_string()),
            Ok(false) => (CheckOutcome::Failed, check.failed_message.to_string()),
            Err(error) => (CheckOutcome::Error, error.to_string()),
        };

        Self {
            id: check.id.to_string(),
            name: check.name,
            title: check.title,
            message,
            result,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ScanSummary {
    pub passed: usize,
    pub failed: usize,
    pub errors: usize,
}

impl ScanSummary {
    pub fn from_checks(checks: &[CheckResult]) -> Self {
        checks.iter().fold(Self::default(), |mut summary, check| {
            match check.result {
                CheckOutcome::Passed => summary.passed += 1,
                CheckOutcome::Failed => summary.failed += 1,
                CheckOutcome::Error => summary.errors += 1,
            }

            summary
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ScanResult {
    /// kesteral version
    pub version: &'static str,
    /// wall-clock time the scan began
    pub timestamp: DateTime<Utc>,
    /// monotonic elapsed time spent scanning
    pub duration_ms: u64,
    /// summary of check results
    pub summary: ScanSummary,
    /// checks and their results
    pub checks: Vec<CheckResult>,
}

impl ScanResult {
    pub fn new(
        checks: Vec<CheckResult>,
        timestamp: Option<DateTime<Utc>>,
        duration_ms: u64,
    ) -> Self {
        let summary = ScanSummary::from_checks(&checks);
        Self {
            timestamp: timestamp.unwrap_or_else(Utc::now),
            duration_ms,
            checks,
            summary,
            version: VERSION,
        }
    }
}

pub fn scan<F>(
    context: &ScanContext,
    system: &(dyn System + Sync),
    on_check_complete: F,
) -> ScanResult
where
    F: Fn() + Sync,
{
    let timestamp = Utc::now();
    let start = Instant::now();

    tracing::info!(
        checks = context.checks.len(),
        parallel = context.parallel,
        "starting scan"
    );
    let run = |check: &'static Check| {
        tracing::info!(check = check.name, "running check");
        let result = (check.run)(system);

        match &result {
            Ok(true) => {
                tracing::info!(check = check.name, "check passed");
            }
            Ok(false) => {
                tracing::info!(check = check.name, "check failed");
            }
            Err(error) => {
                tracing::error!(
                    check = check.name,
                    error = %error,
                    "check encountered an error"
                );
            }
        }

        on_check_complete();
        CheckResult::new(check, result)
    };

    let checks = if context.parallel {
        context.checks.par_iter().map(run).collect()
    } else {
        context.checks.iter().map(run).collect()
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    tracing::info!(duration_ms, "scan completed");

    ScanResult::new(checks, Some(timestamp), duration_ms)
}
