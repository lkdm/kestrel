use crate::{
    checks::{Check, CheckReturnedResult},
    system::{CommandResultExt, System, common::command::CommandRequest},
};

pub static ADMIN_PASSWORD_FOR_PREFERENCES: Check = Check {
    id: "password-modify-preferences",
    description: "An administrator password is required to modify system-wide preferences.",
    recommendation: "Require an administrator password to modify system-wide preferences.",
    run: admin_password_for_preferences,
};

/// Reads an authorizationdb value
fn authorizationdb_value<'a>(output: &'a str, key: &str) -> Option<&'a str> {
    let key = format!("<key>{key}</key>");

    output
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .windows(2)
        .find(|pair| pair[0] == key)
        .map(|pair| pair[1])
        .filter(|value| !value.is_empty())
}

fn admin_password_for_preferences(system: &dyn System) -> CheckReturnedResult {
    let result = system
        .command(&CommandRequest::new(
            "/usr/bin/security",
            &["authorizationdb", "read", "system.preferences"],
        ))
        .ensure_success()?;

    let output = result.stdout_utf8();

    let authenticate_user = authorizationdb_value(&output, "authenticate-user");
    let class = authorizationdb_value(&output, "class");
    let group = authorizationdb_value(&output, "group");

    Ok(authenticate_user == Some("<true/>")
        && class == Some("<string>user</string>")
        && group == Some("<string>admin</string>"))
}

#[cfg(test)]
mod tests {
    use crate::system::{common::command::CommandRequest, test::TestSystem};

    use super::*;

    fn command_output(stdout: &str) -> TestSystem {
        TestSystem::new().with_command(
            CommandRequest::new(
                "/usr/bin/security",
                &["authorizationdb", "read", "system.preferences"],
            )
            .success(stdout),
        )
    }

    #[test]
    fn authorizationdb_value_returns_value_for_key() {
        let output = r#"
            <key>authenticate-user</key>
            <true/>
            <key>class</key>
            <string>user</string>
        "#;

        assert_eq!(
            authorizationdb_value(output, "authenticate-user"),
            Some("<true/>")
        );
        assert_eq!(
            authorizationdb_value(output, "class"),
            Some("<string>user</string>")
        );
    }

    #[test]
    fn authorizationdb_value_returns_none_for_missing_key() {
        let output = r#"
            <key>authenticate-user</key>
            <true/>
        "#;

        assert_eq!(authorizationdb_value(output, "shared"), None);
    }

    #[test]
    fn authorizationdb_value_returns_none_when_key_has_no_value() {
        let output = r#"
            <key>authenticate-user</key>
        "#;

        assert_eq!(authorizationdb_value(output, "authenticate-user"), None);
    }

    #[test]
    fn authorizationdb_value_returns_unexpected_value() {
        let output = r#"
            <key>authenticate-user</key>
            <string>unexpected</string>
        "#;

        assert_eq!(
            authorizationdb_value(output, "authenticate-user"),
            Some("<string>unexpected</string>")
        );
    }

    #[test]
    fn authorizationdb_value_ignores_unrelated_values() {
        let output = r#"
            <key>session-owner</key>
            <false/>
            <key>authenticate-user</key>
            <true/>
        "#;

        assert_eq!(
            authorizationdb_value(output, "authenticate-user"),
            Some("<true/>")
        );
    }

    #[test]
    fn authorizationdb_value_ignores_whitespace() {
        let output = r#"
            <key>authenticate-user</key>
                <true/>
        "#;

        assert_eq!(
            authorizationdb_value(output, "authenticate-user"),
            Some("<true/>")
        );
    }

    #[test]
    fn empty_output_returns_none() {
        assert_eq!(authorizationdb_value("", "authenticate-user"), None);
    }

    #[test]
    fn check_passes_when_admin_authentication_is_required() {
        let system = command_output(
            r#"
                <key>authenticate-user</key>
                <true/>
                <key>class</key>
                <string>user</string>
                <key>group</key>
                <string>admin</string>
            "#,
        );

        let passed =
            admin_password_for_preferences(&system).expect("check should execute successfully");

        assert!(passed);
    }

    #[test]
    fn check_fails_when_authentication_is_not_required() {
        let system = command_output(
            r#"
                <key>authenticate-user</key>
                <false/>
                <key>class</key>
                <string>user</string>
                <key>group</key>
                <string>admin</string>
            "#,
        );

        let passed =
            admin_password_for_preferences(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn check_fails_when_class_is_not_user() {
        let system = command_output(
            r#"
                <key>authenticate-user</key>
                <true/>
                <key>class</key>
                <string>rule</string>
                <key>group</key>
                <string>admin</string>
            "#,
        );

        let passed =
            admin_password_for_preferences(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn check_fails_when_group_is_not_admin() {
        let system = command_output(
            r#"
                <key>authenticate-user</key>
                <true/>
                <key>class</key>
                <string>user</string>
                <key>group</key>
                <string>staff</string>
            "#,
        );

        let passed =
            admin_password_for_preferences(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn check_fails_when_required_property_is_missing() {
        let system = command_output(
            r#"
                <key>authenticate-user</key>
                <true/>
                <key>class</key>
                <string>user</string>
            "#,
        );

        let passed =
            admin_password_for_preferences(&system).expect("check should execute successfully");

        assert!(!passed);
    }

    #[test]
    fn check_propagates_command_error() {
        let system = TestSystem::new();

        let result = admin_password_for_preferences(&system);

        assert!(result.is_err(), "expected command error to propagate");
    }
}
