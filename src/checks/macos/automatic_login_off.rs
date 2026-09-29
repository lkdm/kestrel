use crate::{
    checks::{Check, CheckReturnedResult},
    system::{
        CommandResultExt as _, System, SystemError,
        common::command::{CommandError, CommandRequest},
    },
};

pub static AUTOMATIC_LOGIN_DISABLED: Check = Check {
    name: "automatic-login-disabled",
    description: "Automatic login is disabled.",
    recommendation: "Disable automatic login to require authentication when logging in to macOS.",
    run: automatic_login_disabled,
};

/// checks whether automatic login is disabled
///
/// `defaults read` exits non-zero when the key doesn't exist, meaning automatic login is not configured
fn automatic_login_disabled(system: &dyn System) -> CheckReturnedResult {
    match system.command(&CommandRequest::new(
        "/usr/bin/defaults",
        &[
            "read",
            "/Library/Preferences/com.apple.loginwindow",
            "autoLoginUser",
        ],
    )) {
        Ok(_) => Ok(false),
        Err(SystemError::Command(CommandError::NonZeroExit { stderr, .. }))
            if stderr.contains("does not exist") =>
        {
            Ok(true)
        }
        Err(error) => Err(error.into()),
    }
}
