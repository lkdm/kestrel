use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    checks::{Check, CheckResult, Checks},
    system::System,
};

/// context for starting a scan
#[derive(Debug, Clone)]
pub struct ScanContext {
    /// subset of checks to be scanned
    pub checks: Checks,
}

impl ScanContext {
    pub fn new(checks: &Checks) -> Self {
        Self {
            checks: checks.clone(),
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
    // closure to be called when a check is completed
    // we use this to inform the progress counter of completion
    on_check_complete: F,
) -> ScanResult
where
    F: Fn() + Sync,
{
    let results = context
        .checks
        .par_iter()
        .map(|check| {
            let result = (check.run)(system);
            on_check_complete();
            (*check, result)
        })
        .collect();

    ScanResult { results }
}
