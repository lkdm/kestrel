use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{
        System, SystemError,
        common::command::{CommandError, CommandRequest},
    },
};

pub static HOMEBREW_UPDATED: Check = Check {
    id: check_id!("DFB671BF-6B06-4E00-90C3-D9A8F392F67D"),
    name: "homebrew-updated",
    title: "Require all Homebrew packages to be up to date",
    passed_message: "All Homebrew packages are up to date",
    failed_message: "One or more Homebrew packages are out of date",
    run: homebrew_updated,
};

fn homebrew_updated(system: &dyn System) -> CheckReturnedResult {
    let result = match system.command(&CommandRequest::new("brew", &["outdated", "--quiet"])) {
        // program ran okay
        Ok(result) => result,
        // program is not installed
        Err(SystemError::Command(CommandError::NotFound { .. })) => return Ok(true),
        // some other error
        Err(error) => return Err(error.into()),
    };

    Ok(String::from_utf8_lossy(&result.stdout).trim().is_empty())
}
