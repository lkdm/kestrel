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
    pub id: &'static str,
    pub description: &'static str,
    pub result: CheckOutcome,
}

#[derive(Debug, Serialize)]
pub enum CheckOutcome {
    Passed,
    Failed { recommendation: String },
    Error(String),
}

impl CheckResult {
    pub fn new(check: &Check, result: CheckReturnedResult) -> Self {
        let result = match result {
            Ok(true) => CheckOutcome::Passed,
            Ok(false) => CheckOutcome::Failed {
                recommendation: check.recommendation.to_string(),
            },
            Err(error) => CheckOutcome::Error(error.to_string()),
        };

        Self {
            id: check.id,
            description: check.description,
            result,
        }
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
    /// checks and their results
    pub checks: Vec<CheckResult>,
}

impl ScanResult {
    pub fn new(
        checks: Vec<CheckResult>,
        timestamp: Option<DateTime<Utc>>,
        duration_ms: u64,
    ) -> Self {
        Self {
            timestamp: timestamp.unwrap_or_else(Utc::now),
            duration_ms,
            checks,
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
        tracing::info!(check = check.id, "running check");
        let result = (check.run)(system);

        match &result {
            Ok(true) => {
                tracing::info!(check = check.id, "check passed");
            }
            Ok(false) => {
                tracing::info!(check = check.id, "check failed");
            }
            Err(error) => {
                tracing::error!(
                    check = check.id,
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
