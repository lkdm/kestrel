//! Check that firewall is blocking all incoming connections
use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static FIREWALL_BLOCK_ALL: Check = Check {
    id: check_id!("2E2DFC68-BDF5-463C-BD55-792C28A26E67"),
    name: "firewall-block-incoming",
    description: "The macOS firewall is configured to block all incoming connections.",
    recommendation: "Enable Block All Incoming Connections in the macOS firewall settings.",
    run: firewall_block_all,
};

fn firewall_block_all(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/libexec/ApplicationFirewall/socketfilterfw",
            &["--getblockall"],
        ))
        .ensure_success()?;

    Ok(result.stdout_utf8().contains("is blocking all"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn command_output(stdout: &str) -> TestSystem {
        TestSystem::new().with_command(
            CommandRequest::new(
                "/usr/libexec/ApplicationFirewall/socketfilterfw",
                &["--getblockall"],
            )
            .success(stdout),
        )
    }

    #[test]
    fn firewall_block_all_returns_true_when_firewall_is_blocking_all() {
        let system = command_output("Firewall is blocking all incoming connections.");

        assert!(firewall_block_all(&system).unwrap());
    }

    #[test]
    fn firewall_block_all_returns_false_when_firewall_is_not_blocking_all() {
        let system = command_output("Firewall has block all state set to disabled.");

        assert!(!firewall_block_all(&system).unwrap());
    }

    #[test]
    fn firewall_block_all_returns_false_when_output_does_not_contain_blocking_message() {
        let system = command_output("Unexpected output");

        assert!(!firewall_block_all(&system).unwrap());
    }
}
