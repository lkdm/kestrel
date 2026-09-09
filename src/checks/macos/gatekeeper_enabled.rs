use crate::{
    checks::{Check, CheckResult},
    system::System,
};

fn gatekeeper_enabled(system: &dyn System) -> CheckResult {
    let result = system.command("/usr/sbin/spctl", &["--status"])?;

    Ok(String::from_utf8_lossy(&result.stdout).contains("assessments enabled"))
}

pub static GATEKEEPER_ENABLED: Check = Check {
    id: "macos-gatekeeper-enabled",
    description: "Gatekeeper assessments are enabled.",
    run: gatekeeper_enabled,
};
