use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static REMOTE_LOGIN_DISABLED: Check = Check {
    id: "remote-login-disabled",
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
        let system = TestSystem::new().with_command(
            get_remote_login().failure("systemsetup: requires administrator privileges"),
        );

        let result = remote_login_disabled(&system);

        assert!(
            result.is_err(),
            "expected non-zero systemsetup exit to return an error"
        );
    }
}
