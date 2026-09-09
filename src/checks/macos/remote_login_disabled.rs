use crate::{
    checks::{Check, CheckResult},
    system::System,
};

fn remote_login_disabled(system: &dyn System) -> CheckResult {
    let result = system.command("/usr/sbin/systemsetup", &["-getremotelogin"])?;

    Ok(String::from_utf8_lossy(&result.stdout).contains("Remote Login: Off"))
}

pub static REMOTE_LOGIN_DISABLED: Check = Check {
    id: "macos-remote-login-disabled",
    description: "Remote Login is disabled.",
    run: remote_login_disabled,
};
