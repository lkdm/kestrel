//! Check automatic login is disabled
use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{
        CommandResultExt, System, SystemError,
        common::command::{CommandError, CommandRequest},
    },
};

pub static AUTOMATIC_LOGIN_DISABLED: Check = Check {
    id: check_id!("F066209D-A72C-41F8-9D57-386AE1BDA9A4"),
    name: "automatic-login-disabled",
    title: "Require automatic login to be disabled",
    passed_message: "Automatic login is disabled",
    failed_message: "Automatic login is enabled",
    run: automatic_login_disabled,
};

/// checks whether automatic login is disabled
fn automatic_login_disabled(system: &dyn System) -> CheckReturnedResult {
    match system
        .command(&CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "read",
                "/Library/Preferences/com.apple.loginwindow",
                "autoLoginUser",
            ],
        ))
        .ensure_success()
    {
        Ok(_) => Ok(false),
        Err(SystemError::Command(CommandError::NonZeroExit { stderr, .. }))
            if stderr.contains("does not exist") =>
        {
            // This means autologin is not configured
            Ok(true)
        }
        Err(error) => Err(error.into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::system::common::command::CommandRequest;
    use crate::system::test::TestSystem;

    use super::*;

    fn command() -> CommandRequest {
        CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "read",
                "/Library/Preferences/com.apple.loginwindow",
                "autoLoginUser",
            ],
        )
    }

    #[test]
    fn automatic_login_disabled_returns_false_when_autologin_is_configured() {
        let system = TestSystem::new().with_command(command().success("testuser"));

        assert!(!automatic_login_disabled(&system).unwrap());
    }

    #[test]
    fn automatic_login_disabled_returns_true_when_autologin_is_not_configured() {
        let system = TestSystem::new().with_command(command().failure(
            "The domain/default pair of (/Library/Preferences/com.apple.loginwindow, autoLoginUser) does not exist",
        ));

        assert!(automatic_login_disabled(&system).unwrap());
    }

    #[test]
    fn automatic_login_disabled_propagates_unexpected_command_error() {
        let system = TestSystem::new().with_command(command().failure("Could not read domain"));

        assert!(automatic_login_disabled(&system).is_err());
    }
}
