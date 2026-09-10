use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn docker_rootless(system: &dyn System) -> CheckReturnedResult {
    let result = match system.command("docker", &["info"]) {
        Ok(result) => result,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(true);
        }
        Err(error) => return Err(error),
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

pub static DOCKER_ROOTLESS: Check = Check {
    id: "macos-docker-rootless",
    description: "Docker, if installed, is running in rootless mode.",
    run: docker_rootless,
};
