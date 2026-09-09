use crate::{
    checks::{Check, CheckResult},
    system::System,
};

fn filevault_enabled(system: &dyn System) -> CheckResult {
    let result = system.command("/usr/bin/fdesetup", &["status"])?;

    Ok(String::from_utf8_lossy(&result.stdout).contains("FileVault is On"))
}

pub static FILEVAULT_ENABLED: Check = Check {
    id: "macos-filevault-enabled",
    description: "FileVault disk encryption is enabled.",
    run: filevault_enabled,
};
