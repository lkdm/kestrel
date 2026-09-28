use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn password_after_inactivity(system: &dyn System) -> CheckReturnedResult {
    let result = system.command(
        "/usr/bin/defaults",
        &[
            "-currentHost",
            "read",
            "com.apple.screensaver",
            "askForPassword",
        ],
    )?;

    let output = String::from_utf8_lossy(&result.stdout);

    Ok(output.trim() == "1")
}

pub static PASSWORD_AFTER_INACTIVITY: Check = Check {
    id: "password-after-inactivity",
    description: "A password is required immediately after inactivity.",
    recommendation: "Require a password immediately when the screen saver starts.",
    run: password_after_inactivity,
};
