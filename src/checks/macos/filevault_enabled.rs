use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static FILEVAULT_ENABLED: Check = Check {
    id: check_id!("2A5B7807-F149-4001-89C0-A57886E7F291"),
    name: "filevault-enabled",
    description: "FileVault disk encryption is enabled.",
    recommendation: "Enable FileVault to encrypt the contents of this Mac.",
    run: filevault_enabled,
};

fn filevault_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new("/usr/bin/fdesetup", &["status"]))
        .ensure_success()?;

    Ok(result.stdout_utf8().contains("FileVault is On"))
}
