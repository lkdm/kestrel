//! Checks whether the macOS firewall is enabled
use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static FIREWALL_ENABLED: Check = Check {
    id: check_id!("CFCA04F1-591A-4931-9165-E2C6B34F86C4"),
    name: "firewall-enabled",
    title: "Require the macOS firewall to be enabled",
    passed_message: "The macOS firewall is enabled",
    failed_message: "The macOS firewall is disabled",
    run: firewall_enabled,
};

fn firewall_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/libexec/ApplicationFirewall/socketfilterfw",
            &["--getglobalstate"],
        ))
        .ensure_success()?;

    Ok(result.stdout_utf8().contains("(State = 1)"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn command_output(stdout: &str) -> TestSystem {
        TestSystem::new().with_command(
            CommandRequest::new(
                "/usr/libexec/ApplicationFirewall/socketfilterfw",
                &["--getglobalstate"],
            )
            .success(stdout),
        )
    }

    #[test]
    fn firewall_enabled_returns_true_when_state_is_one() {
        let system = command_output("Firewall is enabled. (State = 1)");

        assert!(firewall_enabled(&system).unwrap());
    }

    #[test]
    fn firewall_enabled_returns_false_when_state_is_zero() {
        let system = command_output("Firewall is disabled. (State = 0)");

        assert!(!firewall_enabled(&system).unwrap());
    }

    #[test]
    fn firewall_enabled_returns_false_when_output_does_not_contain_enabled_message() {
        let system = command_output("Unexpected output");

        assert_eq!(firewall_enabled(&system).unwrap(), false);
    }
}
