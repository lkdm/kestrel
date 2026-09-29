use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static GATEKEEPER_ENABLED: Check = Check {
    name: "gatekeeper-enabled",
    description: "Gatekeeper assessments are enabled.",
    recommendation: "Enable Gatekeeper assessments to allow macOS to verify applications before they run.",
    run: gatekeeper_enabled,
};

fn gatekeeper_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new("/usr/sbin/spctl", &["--status"]))
        .ensure_success()?;

    Ok(result.stdout_utf8().contains("assessments enabled"))
}

#[cfg(test)]
mod tests {
    use crate::system::test::TestSystem;

    use super::*;

    fn spctl_status() -> CommandRequest {
        CommandRequest::new("/usr/sbin/spctl", &["--status"])
    }

    fn command_output(out: &str) -> TestSystem {
        TestSystem::new().with_command(spctl_status().success(out))
    }

    #[test]
    fn passes_when_assessments_are_enabled() {
        let system = command_output("assessments enabled\n");

        let passed = gatekeeper_enabled(&system).expect("check should execute successfully");

        assert!(passed);
    }

    #[test]
    fn fails_when_assessments_are_disabled() {
        let system = command_output("assessments disabled\n");

        let passed = gatekeeper_enabled(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn returns_error_when_spctl_exits_nonzero() {
        let system = TestSystem::new()
            .with_command(spctl_status().failure("spctl: unable to determine status"));

        let result = gatekeeper_enabled(&system);

        assert!(
            result.is_err(),
            "expected non-zero spctl exit to return an error"
        );
    }
}
