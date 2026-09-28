use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn remote_login_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system.command("/usr/sbin/systemsetup", &["-getremotelogin"])?;

    Ok(String::from_utf8_lossy(&result.stdout).contains("Remote Login: Off"))
}

pub static REMOTE_LOGIN_DISABLED: Check = Check {
    id: "remote-login-disabled",
    description: "Remote Login is disabled.",
    recommendation: "Disable Remote Login unless it is required.",
    run: remote_login_disabled,
};
