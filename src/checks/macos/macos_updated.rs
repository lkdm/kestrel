use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static MACOS_UPDATED: Check = Check {
    name: "macos-updated",
    description: "macOS is up to date.",
    recommendation: "Install the latest available macOS updates.",
    run: macos_updated,
};

fn macos_updated(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/sbin/softwareupdate",
            &["--list"],
        ))
        .ensure_success()?;

    Ok(!result.stdout_utf8().contains("Software Update found"))
}

#[cfg(test)]
mod tests {
    use crate::system::test::TestSystem;

    use super::*;

    fn list_updates() -> CommandRequest {
        CommandRequest::new("/usr/sbin/softwareupdate", &["--list"])
    }

    fn command_output(out: &str) -> TestSystem {
        TestSystem::new().with_command(list_updates().success(out))
    }

    #[test]
    fn passes_when_no_updates_are_available() {
        let system = command_output("No new software available.\n");

        let passed = macos_updated(&system).expect("check should execute successfully");

        assert!(passed);
    }

    #[test]
    fn fails_when_updates_are_available() {
        let system = command_output(
            "Software Update found the following new or updated software:\n* macOS Sonoma 14.5\n",
        );

        let passed = macos_updated(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn returns_error_when_softwareupdate_exits_nonzero() {
        let system = TestSystem::new().with_command(
            list_updates().failure("softwareupdate: unable to contact the update server"),
        );

        let result = macos_updated(&system);

        assert!(
            result.is_err(),
            "expected non-zero softwareupdate exit to return an error"
        );
    }
}
