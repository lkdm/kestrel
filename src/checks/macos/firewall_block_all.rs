use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static FIREWALL_BLOCK_ALL: Check = Check {
    id: check_id!("2E2DFC68-BDF5-463C-BD55-792C28A26E67"),
    name: "firewall-block-incoming",
    description: "The macOS firewall is configured to block all incoming connections.",
    recommendation: "Enable Block All Incoming Connections in the macOS firewall settings.",
    run: firewall_block_all,
};

fn firewall_block_all(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/libexec/ApplicationFirewall/socketfilterfw",
            &["--getblockall"],
        ))
        .ensure_success()?;

    Ok(result.stdout_utf8().contains("Block all enabled"))
}
