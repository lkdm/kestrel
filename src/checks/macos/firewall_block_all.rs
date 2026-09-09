use crate::{
    checks::{Check, CheckResult},
    system::System,
};

fn firewall_block_all(system: &dyn System) -> CheckResult {
    let result = system.command(
        "/usr/libexec/ApplicationFirewall/socketfilterfw",
        &["--getblockall"],
    )?;

    Ok(String::from_utf8_lossy(&result.stdout).contains("Block all enabled"))
}

pub static FIREWALL_BLOCK_ALL: Check = Check {
    id: "macos-firewall-block-all",
    description: "The macOS firewall is configured to block all incoming connections.",
    run: firewall_block_all,
};
