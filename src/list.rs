use std::fmt::{self, Display, Formatter};

use crate::checks::{CHECKS, Check};

#[derive(Debug)]
pub struct ListChecksResult {
    pub checks: &'static [Check],
}

impl Display for ListChecksResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for check in self.checks {
            writeln!(f, "{}  {}", check.id, check.description)?;
        }
        Ok(())
    }
}

pub fn list_checks() -> ListChecksResult {
    ListChecksResult { checks: CHECKS }
}
