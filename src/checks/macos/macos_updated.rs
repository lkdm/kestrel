use crate::{
    checks::{Check, CheckResult},
    system::System,
};

fn macos_updated(system: &dyn System) -> CheckResult {
    let result = system.command("/usr/sbin/softwareupdate", &["--list"])?;

    let output = String::from_utf8_lossy(&result.stdout);

    Ok(!output.contains("Software Update found"))
}

pub static MACOS_UPDATED: Check = Check {
    id: "macos-updated",
    description: "macOS is up to date.",
    run: macos_updated,
};
