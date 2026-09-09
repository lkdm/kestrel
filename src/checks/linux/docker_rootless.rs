use crate::{
    checks::{Check, CheckResult},
    system::System,
};

fn docker_rootless(system: &dyn System) -> CheckResult {
    let version = system.command("docker", &["version"])?;

    let version_output = String::from_utf8_lossy(&version.stdout).to_lowercase();

    // Docker is installed but the current user cannot access the daemon.
    // Treat this as passing because Docker is not usable by the user.
    if version_output.contains("cannot connect") && version_output.contains("daemon") {
        return Ok(true);
    }

    let info = system.command("docker", &["info"])?;

    let info_output = String::from_utf8_lossy(&info.stdout).to_lowercase();

    Ok(info_output.contains("rootless"))
}

pub static DOCKER_ROOTLESS: Check = Check {
    id: "linux-docker-rootless",
    description: "Docker is running in rootless mode.",
    run: docker_rootless,
};
