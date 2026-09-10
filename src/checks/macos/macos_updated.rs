use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn macos_updated(system: &dyn System) -> CheckReturnedResult {
    let result = system.command("/usr/sbin/softwareupdate", &["--list"])?;

    let output = String::from_utf8_lossy(&result.stdout);

    Ok(!output.contains("Software Update found"))
}

pub static MACOS_UPDATED: Check = Check {
    id: "macos-updated",
    description: "macOS is up to date.",
    recommendation: "Install the latest available macOS updates.",
    run: macos_updated,
};
