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
    recommendation: "Enable the macOS firewall in System Settings.",
    run: firewall_enabled,
};
