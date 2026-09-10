use rayon::iter::ParallelIterator;

use crate::{
    checks::{Check, CheckResult, Checks},
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

#[derive(Debug)]
pub struct ScanResult {
    pub results: Vec<(&'static Check, CheckResult)>,
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
        (check, result)
    };

    let results = if context.parallel {
        context.checks.par_iter().map(run).collect()
    } else {
        context.checks.iter().map(run).collect()
    };

    tracing::info!("scan completed"); // TODO: add count of passed, failed, errors
    ScanResult { results }
}
