use rayon::iter::ParallelIterator;
use serde::Serialize;

use crate::{
    checks::{Check, CheckReturnedResult, Checks},
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
    pub result: Result<bool, String>,
}

impl CheckResult {
    pub fn new(check: &Check, result: CheckReturnedResult) -> Self {
        Self {
            id: check.id,
            description: check.description,
            result: result.map_err(|error| error.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub checks: Vec<CheckResult>,
}

impl ScanResult {
    pub fn new(checks: Vec<CheckResult>) -> Self {
        Self { checks }
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

    tracing::info!("scan completed"); // TODO: add count of passed, failed, errors

    ScanResult::new(checks)
}
