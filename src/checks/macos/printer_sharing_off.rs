use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static PRINTER_SHARING_DISABLED: Check = Check {
    id: check_id!("447883E3-AB02-4880-A8A0-7CB16F30BB75"),
    name: "printer-sharing-disabled",
    description: "Printer Sharing is disabled.",
    recommendation: "Disable Printer Sharing when it is not needed.",
    run: printer_sharing_disabled,
};

fn printer_sharing_disabled(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/sbin/system_profiler",
            &["SPPrintersDataType"],
        ))
        .ensure_success()?;

    Ok(result
        .stdout_utf8()
        .lines()
        .any(|line| line.trim() == "System Printer Sharing: No"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn printers() -> CommandRequest {
        CommandRequest::new("/usr/sbin/system_profiler", &["SPPrintersDataType"])
    }

    fn command_output(output: &str) -> TestSystem {
        TestSystem::new().with_command(printers().success(output))
    }

    #[test]
    fn passes_when_printer_sharing_is_disabled() {
        let system = command_output(
            r#"
                Printers:

                    Brother HL-L3270CDW series:

                      Status: Offline
                      Print Server: Local
                      System Printer Sharing: No
                      Shared: No
            "#,
        );

        assert!(printer_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_printer_sharing_is_enabled() {
        let system = command_output(
            r#"
                Printers:

                    Brother HL-L3270CDW series:

                      Status: Offline
                      Print Server: Local
                      System Printer Sharing: Yes
                      Shared: Yes
            "#,
        );

        assert!(!printer_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn fails_when_printer_sharing_status_is_missing() {
        let system = command_output(
            r#"
                Printers:

                    Brother HL-L3270CDW series:

                      Status: Offline
                      Print Server: Local
                      Shared: No
            "#,
        );

        assert!(!printer_sharing_disabled(&system).unwrap());
    }

    #[test]
    fn propagates_system_profiler_error() {
        let system = TestSystem::new().with_command(printers().failure("system_profiler failed"));

        assert!(printer_sharing_disabled(&system).is_err());
    }
}
