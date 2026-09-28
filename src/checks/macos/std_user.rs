use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

pub static STANDARD_USER: Check = Check {
    id: "standard-user",
    description: "The daily user account is a Standard user rather than an Administrator.",
    recommendation: "Use a Standard user account for daily activities instead of an Administrator account.",
    run: user_in_admin_group,
};

fn user_in_admin_group(system: &dyn System) -> CheckReturnedResult {
    let result = system.command("/usr/bin/id", &["-Gn"])?;

    if !result.success() {
        return Err(std::io::Error::other("id command exited unsuccessfully"));
    }

    let output = String::from_utf8_lossy(&result.stdout);

    Ok(!output.split_whitespace().any(|group| group == "admin"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandResult, test::TestSystem};

    use super::*;

    #[test]
    fn standard_user_is_standard() {
        let system = TestSystem::new().command_stdout("/usr/bin/id", &["-Gn"], "staff everyone");

        let passed =
            user_in_admin_group(&system).expect("standard_user should execute successfully");

        assert!(passed, "expected standard user without admin group");
    }

    #[test]
    fn standard_user_is_admin() {
        let system =
            TestSystem::new().command_stdout("/usr/bin/id", &["-Gn"], "staff admin everyone");

        let passed =
            user_in_admin_group(&system).expect("standard_user should execute successfully");

        assert!(!passed, "expected administrator with admin group to fail");
    }

    #[test]
    fn standard_user_does_not_match_similar_group_name() {
        let system =
            TestSystem::new().command_stdout("/usr/bin/id", &["-Gn"], "staff adminusers everyone");

        let passed =
            user_in_admin_group(&system).expect("standard_user should execute successfully");

        assert!(
            passed,
            "group names containing 'admin' should not count as the admin group"
        );
    }

    #[test]
    fn standard_user_returns_error_when_id_fails() {
        let system = {
            let this = TestSystem::new();
            let args: &[&str] = &["-Gn"];
            this.command(
                "/usr/bin/id",
                args,
                CommandResult::test_failure("id: failed to determine groups"),
            )
        };

        let result = user_in_admin_group(&system);

        assert!(
            result.is_err(),
            "standard_user should return an error when id fails"
        );
    }

    #[test]
    fn standard_user_with_empty_group_output_is_standard() {
        let system = TestSystem::new().command_stdout("/usr/bin/id", &["-Gn"], "");

        let passed =
            user_in_admin_group(&system).expect("standard_user should execute successfully");

        assert!(passed);
    }
}
