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
    let run = |check: &'static Check| {
        let result = (check.run)(system);
        on_check_complete();
        (check, result)
    };

    let results = if context.parallel {
        context.checks.par_iter().map(run).collect()
    } else {
        context.checks.iter().map(run).collect()
    };

    ScanResult { results }
}
