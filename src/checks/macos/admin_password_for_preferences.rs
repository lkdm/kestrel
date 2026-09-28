use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn admin_password_for_preferences(system: &dyn System) -> CheckReturnedResult {
    let result = system.command(
        "/usr/bin/security",
        &["authorizationdb", "read", "system.preferences"],
    )?;

    let output = String::from_utf8_lossy(&result.stdout);

    // `shared = false` means the authorization must be
    // performed for each access to system-wide preferences.
    Ok(output.lines().any(|line| line.trim() == "<false/>"))
}

pub static ADMIN_PASSWORD_FOR_PREFERENCES: Check = Check {
    id: "macos-admin-password-for-preferences",
    description: "An administrator password is required to access system-wide preferences.",
    recommendation: "Require an administrator password to access system-wide preferences.",
    run: admin_password_for_preferences,
};
