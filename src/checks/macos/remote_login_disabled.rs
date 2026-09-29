//! Check that remote login is disabled
use crate::{
    check_id,
    checks::{Check, CheckError, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static REMOTE_LOGIN_DISABLED: Check = Check {
    id: check_id!("25BC191F-171A-47DB-BDF9-64573497054B"),
    name: "remote-login-disabled",
    description: "Remote Login is disabled.",
    recommendation: "Disable Remote Login unless it is required.",
    run: remote_login_disabled,
};

fn remote_login_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/sbin/systemsetup",
            &["-getremotelogin"],
        ))
        .ensure_success()?;

    if result
        .stdout_utf8()
        .contains("You need administrator access")
    {
        return Err(CheckError::RequiresAdmin);
    }

    Ok(result.stdout_utf8().contains("Remote Login: Off"))
}

#[cfg(test)]
mod tests {
    use crate::system::test::TestSystem;

    use super::*;

    fn get_remote_login() -> CommandRequest {
        CommandRequest::new("/usr/sbin/systemsetup", &["-getremotelogin"])
    }

    fn command_output(out: &str) -> TestSystem {
        TestSystem::new().with_command(get_remote_login().success(out))
    }

    #[test]
    fn passes_when_remote_login_is_off() {
        let system = command_output("Remote Login: Off\n");

        let passed = remote_login_disabled(&system).expect("check should execute successfully");

        assert!(passed);
    }

    #[test]
    fn fails_when_remote_login_is_on() {
        let system = command_output("Remote Login: On\n");

        let passed = remote_login_disabled(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn returns_error_when_systemsetup_exits_nonzero() {
        let system =
            TestSystem::new().with_command(get_remote_login().failure("some unexpected error"));

        let result = remote_login_disabled(&system);

        assert!(result.is_err());
    }

    #[test]
    fn returns_requires_admin_when_systemsetup_requires_administrator_access() {
        let system = TestSystem::new().with_command(
            get_remote_login()
                .success("You need administrator access to run this tool... exiting!"),
        );

        let result = remote_login_disabled(&system);

        assert!(matches!(result, Err(CheckError::RequiresAdmin)));
    }
}
