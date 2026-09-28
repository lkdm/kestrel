use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static PASSWORD_AFTER_INACTIVITY: Check = Check {
    id: "password-after-inactivity",
    description: "A password is required immediately after inactivity.",
    recommendation: "Require a password immediately when the screen saver starts.",
    run: password_after_inactivity,
};
// TODO: what do you want password_after_inactivity to do when the askForPassword key has never been set?
fn password_after_inactivity(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "-currentHost",
                "read",
                "com.apple.screensaver",
                "askForPassword",
            ],
        ))
        .ensure_success()?;

    Ok(result.stdout_utf8().trim() == "1")
}

#[cfg(test)]
mod tests {
    use crate::system::test::TestSystem;

    use super::*;

    fn ask_for_password() -> CommandRequest {
        CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "-currentHost",
                "read",
                "com.apple.screensaver",
                "askForPassword",
            ],
        )
    }

    fn command_output(out: &str) -> TestSystem {
        TestSystem::new().with_command(ask_for_password().success(out))
    }

    #[test]
    fn passes_when_ask_for_password_is_enabled() {
        let system = command_output("1\n");

        let passed = password_after_inactivity(&system).expect("check should execute successfully");

        assert!(passed);
    }

    #[test]
    fn fails_when_ask_for_password_is_disabled() {
        let system = command_output("0\n");

        let passed = password_after_inactivity(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn fails_on_unexpected_output() {
        let system = command_output("");

        let passed = password_after_inactivity(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn returns_error_when_defaults_exits_nonzero() {
        let system = TestSystem::new().with_command(
            ask_for_password().failure("defaults: the domain/default pair does not exist"),
        );

        let result = password_after_inactivity(&system);

        assert!(
            result.is_err(),
            "expected non-zero defaults exit to return an error"
        );
    }
}
