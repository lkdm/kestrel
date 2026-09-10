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

pub fn scan(context: &ScanContext, system: &(dyn System + Sync)) -> ScanResult {
    let results = context
        .checks
        .par_iter()
        .map(|check| {
            let result = (check.run)(system);
            (*check, result)
        })
        .collect();

    ScanResult { results }
}
