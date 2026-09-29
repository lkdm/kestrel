use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt, System, common::command::CommandRequest},
};

pub static SIP_ENABLED: Check = Check {
    name: "sip-enabled",
    description: "System Integrity Protection is enabled.",
    recommendation: "Enable System Integrity Protection to protect critical macOS system files and settings.",
    run: sip_enabled,
};

fn sip_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new("/usr/bin/csrutil", &["status"]))
        .ensure_success()?;

    Ok(result
        .stdout_utf8()
        .contains("System Integrity Protection status: enabled"))
}

#[cfg(test)]
mod tests {
    use crate::system::test::TestSystem;

    use super::*;

    fn csrutil_status() -> CommandRequest {
        CommandRequest::new("/usr/bin/csrutil", &["status"])
    }

    fn command_output(out: &str) -> TestSystem {
        TestSystem::new().with_command(csrutil_status().success(out))
    }

    #[test]
    fn sip_enabled_when_status_reports_enabled() {
        let system = command_output("System Integrity Protection status: enabled.");

        let passed = sip_enabled(&system).expect("check should execute successfully");

        assert!(passed);
    }

    #[test]
    fn sip_disabled_when_status_reports_disabled() {
        let system = command_output("System Integrity Protection status: disabled.");

        let passed = sip_enabled(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn sip_disabled_on_unexpected_output() {
        let system = command_output("csrutil: unrecognized command\n");

        let passed = sip_enabled(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn returns_error_when_csrutil_exits_nonzero() {
        let system =
            TestSystem::new().with_command(csrutil_status().failure("csrutil: command failed"));

        let result = sip_enabled(&system);

        assert!(
            result.is_err(),
            "expected non-zero csrutil exit to return an error"
        );
    }
}
