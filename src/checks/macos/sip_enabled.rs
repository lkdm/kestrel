use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn sip_enabled(system: &dyn System) -> CheckReturnedResult {
    let result = system.command("/usr/bin/csrutil", &["status"])?;

    let output = String::from_utf8_lossy(&result.stdout);

    Ok(output.contains("System Integrity Protection status: enabled"))
}

pub static SIP_ENABLED: Check = Check {
    id: "macos-sip-enabled",
    description: "System Integrity Protection is enabled.",
    run: sip_enabled,
};
