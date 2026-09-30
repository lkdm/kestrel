use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static MEDIA_SHARING_DISABLED: Check = Check {
    id: check_id!("D3613AC3-269F-43EB-B67D-52DA249C9E1A"),
    name: "media-sharing-disabled",
    title: "Require Media Sharing to be disabled",
    passed_message: "Media Sharing is disabled",
    failed_message: "Media Sharing is enabled",
    run: media_sharing_disabled,
};

fn media_sharing_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/bin/defaults",
            &["read", "com.apple.amp.mediasharingd"],
        ))
        .ensure_success()?;

    let stdout = result.stdout_utf8();

    let home_sharing_disabled = stdout
        .lines()
        .any(|line| line.trim() == "\"home-sharing-enabled\" = 0;");

    let public_sharing_disabled = stdout
        .lines()
        .any(|line| line.trim() == "\"public-sharing-enabled\" = 0;");

    Ok(home_sharing_disabled && public_sharing_disabled)
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn media_sharing() -> CommandRequest {
        CommandRequest::new(
            "/usr/bin/defaults",
            &["read", "com.apple.amp.mediasharingd"],
        )
    }

    fn command_output(output: &str) -> TestSystem {
        TestSystem::new().with_command(media_sharing().success(output))
    }

    #[test]
    fn passes_when_media_sharing_is_disabled() {
        let system = command_output(
            r#"
                {
                    "home-sharing-enabled" = 0;
                    "public-sharing-enabled" = 0;
                }
            "#,
        );

        assert!(media_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_home_sharing_is_enabled() {
        let system = command_output(
            r#"
                {
                    "home-sharing-enabled" = 1;
                    "public-sharing-enabled" = 0;
                }
            "#,
        );

        assert!(!media_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_public_sharing_is_enabled() {
        let system = command_output(
            r#"
                {
                    "home-sharing-enabled" = 0;
                    "public-sharing-enabled" = 1;
                }
            "#,
        );

        assert!(!media_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_sharing_status_is_missing() {
        let system = command_output(
            r#"
                {
                    "photo-sharing-enabled" = 0;
                }
            "#,
        );

        assert!(!media_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn propagates_defaults_error() {
        let system =
            TestSystem::new().with_command(media_sharing().failure("defaults read failed"));

        assert!(media_sharing_disabled(&system).is_err());
    }
}
