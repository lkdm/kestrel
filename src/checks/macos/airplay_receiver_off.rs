use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static AIRPLAY_RECEIVER_DISABLED: Check = Check {
    id: check_id!("1B2AF42D-A79A-4B1D-AD86-3F885F17AF8B"),
    name: "airplay-receiver-disabled",
    title: "Require AirPlay Receiver to be disabled",
    passed_message: "AirPlay Receiver is disabled",
    failed_message: "AirPlay Receiver is enabled",
    run: airplay_receiver_disabled,
};

fn airplay_receiver_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "-currentHost",
                "read",
                "com.apple.controlcenter",
                "AirplayReceiverEnabled",
            ],
        ))
        .ensure_success()?;

    Ok(result.stdout_utf8().trim() != "1")
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn airplay_receiver() -> CommandRequest {
        CommandRequest::new(
            "/usr/bin/defaults",
            &[
                "-currentHost",
                "read",
                "com.apple.controlcenter",
                "AirplayReceiverEnabled",
            ],
        )
    }

    fn command_output(output: &str) -> TestSystem {
        TestSystem::new().with_command(airplay_receiver().success(output))
    }

    #[test]
    fn passes_when_airplay_receiver_is_disabled() {
        let system = command_output("0\n");

        assert!(airplay_receiver_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_airplay_receiver_is_enabled() {
        let system = command_output("1\n");

        assert!(!airplay_receiver_disabled(&system).unwrap());
    }

    #[test]
    fn passes_when_airplay_receiver_has_unexpected_disabled_value() {
        let system = command_output("2\n");

        assert!(airplay_receiver_disabled(&system).unwrap());
    }

    #[test]
    fn propagates_defaults_error() {
        let system =
            TestSystem::new().with_command(airplay_receiver().failure("defaults read failed"));

        assert!(airplay_receiver_disabled(&system).is_err());
    }
}
