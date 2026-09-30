use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{
        CommandResultExt as _, System, SystemError,
        common::command::{CommandError, CommandRequest},
    },
};

pub static PASSWORD_AFTER_INACTIVITY: Check = Check {
    id: check_id!("7F4A48C8-8DF0-4AD4-A95D-E2AC3D6A74B1"),
    name: "password-after-inactivity",
    title: "Require a password immediately after inactivity",
    passed_message: "A password is required immediately when the screen saver starts",
    failed_message: "A password is not required immediately when the screen saver starts",
    run: password_after_inactivity,
};
// TODO: what do you want password_after_inactivity to do when the askForPassword key has never been set?
fn password_after_inactivity(system: &dyn System) -> CheckReturnedResult {
    match system.command(&CommandRequest::new(
        "/usr/bin/defaults",
        &[
            "-currentHost",
            "read",
            "com.apple.screensaver",
            "askForPassword",
        ],
    )) {
        Ok(result) => Ok(result.stdout_utf8().trim() == "1"),
        Err(SystemError::Command(CommandError::NonZeroExit { stderr, .. }))
            if stderr.contains("does not exist") =>
        {
            Ok(false)
        }
        Err(error) => Err(error.into()),
    }
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
}
