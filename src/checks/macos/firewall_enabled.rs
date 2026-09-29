use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static FIREWALL_ENABLED: Check = Check {
    id: check_id!("CFCA04F1-591A-4931-9165-E2C6B34F86C4"),
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
