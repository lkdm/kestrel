use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static INTERNET_SHARING_DISABLED: Check = Check {
    id: check_id!("E7758D2B-92AD-44C4-8B81-8BC4E7961296"),
    name: "internet-sharing-disabled",
    description: "Internet Sharing is disabled.",
    recommendation: "Disable Internet Sharing when it is not needed.",
    run: internet_sharing_disabled,
};

fn internet_sharing_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "read",
                "/Library/Preferences/SystemConfiguration/com.apple.nat",
                "NAT",
            ],
        ))
        .ensure_success()?;

    Ok(!result
        .stdout_utf8()
        .lines()
        .any(|line| line.trim() == "Enabled = 1;"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn internet_sharing() -> CommandRequest {
        CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "read",
                "/Library/Preferences/SystemConfiguration/com.apple.nat",
                "NAT",
            ],
        )
    }

    fn command_output(output: &str) -> TestSystem {
        TestSystem::new().with_command(internet_sharing().success(output))
    }

    #[test]
    fn passes_when_internet_sharing_is_disabled() {
        let system = command_output(
            r#"
                {
                    Enabled = 0;
                    NatPortMapDisabled = 0;
                }
            "#,
        );

        assert!(internet_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_internet_sharing_is_enabled() {
        let system = command_output(
            r#"
                {
                    Enabled = 1;
                    NatPortMapDisabled = 0;
                }
            "#,
        );

        assert!(!internet_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn passes_when_nat_enabled_is_missing() {
        let system = command_output(
            r#"
                {
                    NatPortMapDisabled = 0;
                }
            "#,
        );

        assert!(internet_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn propagates_defaults_error() {
        let system =
            TestSystem::new().with_command(internet_sharing().failure("defaults read failed"));

        assert!(internet_sharing_disabled(&system).is_err());
    }
}
