use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static FIREWALL_ENABLED: Check = Check {
    name: "firewall-enabled",
    description: "The macOS firewall is enabled.",
    recommendation: "Enable the macOS firewall in System Settings.",
    run: firewall_enabled,
};

fn firewall_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/libexec/ApplicationFirewall/socketfilterfw",
            &["--getglobalstate"],
        ))
        .ensure_success()?;

    Ok(result.stdout_utf8().contains("Firewall is enabled"))
}
