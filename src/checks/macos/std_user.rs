use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt as _, System, common::command::CommandRequest},
};

pub static STANDARD_USER: Check = Check {
    name: "standard-user",
    description: "The daily user account is a Standard user rather than an Administrator.",
    recommendation: "Use a Standard user account for daily activities instead of an Administrator account.",
    run: is_standard_user,
};

fn is_standard_user(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new("/usr/bin/id", &["-Gn"]))
        .ensure_success()?;

    Ok(!result
        .stdout_utf8()
        .split_whitespace()
        .any(|group| group == "admin"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn id_groups() -> CommandRequest {
        CommandRequest::new("/usr/bin/id", &["-Gn"])
    }

    fn command_output(out: &str) -> TestSystem {
        TestSystem::new().with_command(id_groups().success(out))
    }

    #[test]
    fn standard_user_is_standard() {
        let system = command_output("staff everyone");
        assert!(is_standard_user(&system).expect("should execute"));
    }

    #[test]
    fn standard_user_is_admin() {
        let system = command_output("staff admin everyone");
        assert!(!is_standard_user(&system).expect("should execute"));
    }

    #[test]
    fn standard_user_returns_error_when_id_exits_nonzero() {
        let system = TestSystem::new().with_command(id_groups().failure("id: failed"));
        assert!(is_standard_user(&system).is_err());
    }
}
