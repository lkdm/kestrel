use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static AIRDROP_SECURED: Check = Check {
    id: check_id!("CAE45547-CD42-4D11-8224-23C23204DDBD"),
    name: "airdrop-secured",
    title: "Require AirDrop to be restricted to Contacts Only or disabled",
    passed_message: "AirDrop is restricted to Contacts Only or disabled",
    failed_message: "AirDrop is not restricted to Contacts Only or disabled",
    run: airdrop_secured,
};

fn airdrop_secured(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/bin/defaults",
            &["read", "com.apple.sharingd", "DiscoverableMode"],
        ))
        .ensure_success()?;

    Ok(matches!(
        result.stdout_utf8().trim(),
        "Contacts Only" | "Off"
    ))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn discoverable_mode() -> CommandRequest {
        CommandRequest::new(
            "/usr/bin/defaults",
            &["read", "com.apple.sharingd", "DiscoverableMode"],
        )
    }

    fn command_output(output: &str) -> TestSystem {
        TestSystem::new().with_command(discoverable_mode().success(output))
    }

    #[test]
    fn passes_when_airdrop_is_contacts_only() {
        let system = command_output("Contacts Only\n");

        assert!(airdrop_secured(&system).unwrap());
    }

    #[test]
    fn passes_when_airdrop_is_off() {
        let system = command_output("Off\n");

        assert!(airdrop_secured(&system).unwrap());
    }

    #[test]
    fn fails_when_airdrop_allows_everyone() {
        let system = command_output("Everyone\n");

        assert!(!airdrop_secured(&system).unwrap());
    }

    #[test]
    fn fails_for_unknown_discoverable_mode() {
        let system = command_output("Unknown\n");

        assert!(!airdrop_secured(&system).unwrap());
    }

    #[test]
    fn propagates_defaults_error() {
        let system = TestSystem::new().with_command(discoverable_mode().failure(
            "The domain/default pair of (com.apple.sharingd, DiscoverableMode) does not exist",
        ));

        assert!(airdrop_secured(&system).is_err());
    }
}
