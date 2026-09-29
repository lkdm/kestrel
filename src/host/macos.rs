use uuid::Uuid;

use crate::system::{System, common::command::CommandRequest};

pub fn platform_uuid(system: &dyn System) -> Result<Uuid, uuid::Error> {
    let output = system.command(&CommandRequest::new(
        "/usr/sbin/ioreg",
        &["-rd1", "-c", "IOPlatformExpertDevice"],
    ))?;
}
