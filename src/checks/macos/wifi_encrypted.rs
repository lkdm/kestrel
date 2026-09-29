//! Check WiFi is encrypted
use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static WIFI_ENCRYPTED: Check = Check {
    id: check_id!("86F3D3A1-5BB6-4D4E-A070-0461F4A37AA8"),
    name: "wifi-encrypted",
    description: "The Mac is connected to an encrypted Wi-Fi network.",
    recommendation: "Connect to a Wi-Fi network that uses encryption.",
    run: wifi_encrypted,
};

fn wifi_encrypted(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/sbin/system_profiler",
            &["SPAirPortDataType"],
        ))
        .ensure_success()?;

    Ok(wifi_is_encrypted(&result.stdout_utf8()))
}

fn wifi_is_encrypted(output: &str) -> bool {
    let mut connected = false;
    let mut current_network = false;

    for line in output.lines() {
        let line = line.trim();

        if line == "Status: Connected" {
            connected = true;
            continue;
        }

        if connected && line == "Current Network Information:" {
            current_network = true;
            continue;
        }

        if current_network && line.starts_with("Security:") {
            let security = line.strip_prefix("Security:").unwrap().trim();

            return !matches!(security, "None" | "Open");
        }

        if current_network && line == "Other Local Wi-Fi Networks:" {
            break;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn airport_data(output: &str) -> TestSystem {
        TestSystem::new().with_command(
            CommandRequest::new("/usr/sbin/system_profiler", &["SPAirPortDataType"])
                .success(output),
        )
    }

    #[test]
    fn passes_when_connected_wifi_is_encrypted() {
        let system = airport_data(
            r#"
                Wi-Fi:
                    Interfaces:
                        en0:
                            Status: Connected
                            Current Network Information:
                                My Network:
                                    PHY Mode: 802.11ax
                                    Security: WPA2/WPA3 Personal
                                    Signal / Noise: -75 dBm / -90 dBm
                            Other Local Wi-Fi Networks:
                                Open Network:
                                    Security: None
            "#,
        );

        assert!(wifi_encrypted(&system).unwrap());
    }

    #[test]
    fn fails_when_connected_wifi_is_open() {
        let system = airport_data(
            r#"
                Wi-Fi:
                    Interfaces:
                        en0:
                            Status: Connected
                            Current Network Information:
                                Coffee Shop:
                                    PHY Mode: 802.11ac
                                    Security: None
                            Other Local Wi-Fi Networks:
                                Home Network:
                                    Security: WPA2 Personal
            "#,
        );

        assert!(!wifi_encrypted(&system).unwrap());
    }

    #[test]
    fn fails_when_connected_wifi_has_open_security() {
        let system = airport_data(
            r#"
                Wi-Fi:
                    Interfaces:
                        en0:
                            Status: Connected
                            Current Network Information:
                                Coffee Shop:
                                    Security: Open
            "#,
        );

        assert!(!wifi_encrypted(&system).unwrap());
    }

    #[test]
    fn ignores_security_of_other_wifi_networks() {
        let system = airport_data(
            r#"
                Wi-Fi:
                    Interfaces:
                        en0:
                            Status: Connected
                            Current Network Information:
                                My Network:
                                    Security: WPA2/WPA3 Personal
                            Other Local Wi-Fi Networks:
                                Open Network:
                                    Security: None
            "#,
        );

        assert!(wifi_encrypted(&system).unwrap());
    }

    #[test]
    fn fails_when_not_connected_to_wifi() {
        let system = airport_data(
            r#"
                Wi-Fi:
                    Interfaces:
                        en0:
                            Status: Not Connected
                            Other Local Wi-Fi Networks:
                                Open Network:
                                    Security: None
            "#,
        );

        assert!(!wifi_encrypted(&system).unwrap());
    }

    #[test]
    fn fails_when_security_information_is_missing() {
        let system = airport_data(
            r#"
                Wi-Fi:
                    Interfaces:
                        en0:
                            Status: Connected
                            Current Network Information:
                                My Network:
                                    PHY Mode: 802.11ax
                                    Channel: 157 (5GHz, 80MHz)
            "#,
        );

        assert!(!wifi_encrypted(&system).unwrap());
    }

    #[test]
    fn propagates_system_profiler_error() {
        let system = TestSystem::new().with_command(
            CommandRequest::new("/usr/sbin/system_profiler", &["SPAirPortDataType"])
                .failure("system_profiler failed"),
        );

        assert!(wifi_encrypted(&system).is_err());
    }
}
