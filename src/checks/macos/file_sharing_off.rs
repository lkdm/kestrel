//! Check that SMB file sharing is disabled
use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static SMB_FILE_SHARING_DISABLED: Check = Check {
    id: check_id!("47E6929E-D7E7-4B72-ABF1-755E06B7541B"),
    name: "smb-file-sharing-disabled",
    title: "Require SMB File Sharing to be disabled",
    passed_message: "SMB File Sharing is disabled",
    failed_message: "SMB File Sharing is enabled",
    run: smb_file_sharing_disabled,
};

// TODO: ideally this should look at the configuration item, instead of whether or not
fn smb_file_sharing_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/bin/launchctl",
            &["print-disabled", "system"],
        ))
        .ensure_success()?;

    Ok(result
        .stdout_utf8()
        .lines()
        .any(|line| line.trim() == "\"com.apple.smbd\" => disabled"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn command_output(stdout: &str) -> TestSystem {
        TestSystem::new().with_command(
            CommandRequest::new("/bin/launchctl", &["print-disabled", "system"]).success(stdout),
        )
    }

    #[test]
    fn passes_when_smb_is_disabled() {
        let system = command_output(
            r#"
            disabled services = {
                "com.apple.smbd" => disabled
            }
        "#,
        );

        assert!(smb_file_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_smb_is_enabled() {
        let system = command_output(
            r#"
            disabled services = {
                "com.apple.smbd" => enabled
            }
        "#,
        );

        assert!(!smb_file_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_smb_status_is_missing() {
        let system = command_output(
            r#"
            disabled services = {
                "com.apple.ftpd" => disabled
            }
        "#,
        );

        assert!(!smb_file_sharing_disabled(&system).unwrap());
    }
}
