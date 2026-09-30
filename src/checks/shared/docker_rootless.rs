use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{
        CommandResultExt as _, System, SystemError,
        common::command::{CommandError, CommandRequest},
    },
};

pub static DOCKER_ROOTLESS: Check = Check {
    id: check_id!("E3AF8C1C-8EA0-4155-AB27-B9EB50412B37"),
    name: "docker-rootless",
    title: "Require Docker to run in rootless mode",
    passed_message: "Docker is running in rootless mode",
    failed_message: "Docker is not running in rootless mode",
    run: docker_rootless,
};

fn docker_rootless(system: &dyn System) -> CheckReturnedResult {
    let result = match system.command(&CommandRequest::new("docker", &["info"])) {
        // `docker info` ran okay
        Ok(result) => result,
        // pass if docker is not installed
        Err(SystemError::Command(CommandError::NotFound { .. })) => return Ok(true),
        // some other error
        Err(error) => return Err(error.into()),
    };

    let output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr),
    )
    .to_lowercase();

    if output.contains("cannot connect") && output.contains("daemon") {
        return Ok(true);
    }

    Ok(output.contains("rootless"))
}
