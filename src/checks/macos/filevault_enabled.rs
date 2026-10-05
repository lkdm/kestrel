//! Check that FileVault is enabled
use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static FILEVAULT_ENABLED: Check = Check {
    id: check_id!("2A5B7807-F149-4001-89C0-A57886E7F291"),
    name: "filevault-enabled",
    title: "Require FileVault disk encryption to be enabled",
    passed_message: "FileVault disk encryption is enabled",
    failed_message: "FileVault disk encryption is not enabled",
    run: filevault_enabled,
};

fn filevault_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new("/usr/bin/fdesetup", &["status"])) // TODO: add -extended to get more information about volume
        .ensure_success()?;

    Ok(result.stdout_utf8().contains("FileVault is On"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn command_output(stdout: &str) -> TestSystem {
        TestSystem::new()
            .with_command(CommandRequest::new("/usr/bin/fdesetup", &["status"]).success(stdout))
    }

    #[test]
    fn filevault_enabled_returns_true_when_filevault_is_on() {
        let system = command_output("FileVault is On.");

        assert!(filevault_enabled(&system).unwrap());
    }

    #[test]
    fn filevault_enabled_returns_false_when_filevault_is_off() {
        let system = command_output("FileVault is Off.");

        assert!(!filevault_enabled(&system).unwrap());
    }

    #[test]
    fn filevault_enabled_returns_false_when_output_does_not_contain_filevault_status() {
        let system = command_output("Unexpected output");

        assert!(!filevault_enabled(&system).unwrap());
    }
}
