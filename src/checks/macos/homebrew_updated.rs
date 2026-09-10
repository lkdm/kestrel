use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn homebrew_updated(system: &dyn System) -> CheckReturnedResult {
    let result = match system.command("brew", &["outdated", "--quiet"]) {
        Ok(result) => result,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(true);
        }
        Err(error) => return Err(error),
    };

    Ok(String::from_utf8_lossy(&result.stdout).trim().is_empty())
}

pub static HOMEBREW_UPDATED: Check = Check {
    id: "macos-homebrew-updated",
    description: "All Homebrew packages are up to date.",
    run: homebrew_updated,
};
