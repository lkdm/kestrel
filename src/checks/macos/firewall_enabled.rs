use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn firewall_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system.command(
        "/usr/libexec/ApplicationFirewall/socketfilterfw",
        &["--getglobalstate"],
    )?;

    Ok(String::from_utf8_lossy(&result.stdout).contains("Firewall is enabled"))
}

pub static FIREWALL_ENABLED: Check = Check {
    id: "macos-firewall-enabled",
    description: "The macOS firewall is enabled.",
    run: firewall_enabled,
};
