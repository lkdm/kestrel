use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn automatic_login_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system.command(
        "/usr/bin/defaults",
        &[
            "read",
            "/Library/Preferences/com.apple.loginwindow",
            "autoLoginUser",
        ],
    );

    match result {
        Ok(result) => {
            let output = String::from_utf8_lossy(&result.stdout);
            Ok(output.trim().is_empty())
        }
        // `defaults read` exits non-zero when the key doesn't exist,
        // which means automatic login is not configured.
        Err(_) => Ok(true),
    }
}

pub static AUTOMATIC_LOGIN_DISABLED: Check = Check {
    id: "macos-automatic-login-disabled",
    description: "Automatic login is disabled.",
    recommendation: "Disable automatic login to require authentication when logging in to macOS.",
    run: automatic_login_disabled,
};
