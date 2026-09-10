use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn gatekeeper_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system.command("/usr/sbin/spctl", &["--status"])?;

    Ok(String::from_utf8_lossy(&result.stdout).contains("assessments enabled"))
}

pub static GATEKEEPER_ENABLED: Check = Check {
    id: "macos-gatekeeper-enabled",
    description: "Gatekeeper assessments are enabled.",
    recommendation: "Enable Gatekeeper assessments to allow macOS to verify applications before they run.",
    run: gatekeeper_enabled,
};
