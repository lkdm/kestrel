use crate::system::System;

#[derive(Debug)]
pub struct Check {
    pub id: &'static str,
    pub description: &'static str,
    pub query: Query,
    pub assertion: Assertion,
}

#[derive(Debug)]
pub enum Query {
    /// check if file exists
    FileExists { path: &'static str },
    /// run a command
    Command {
        program: &'static str,
        args: &'static [&'static str],
    },
    /// check linux configuration
    Sysctl { key: &'static str },
    /// check windows configuration
    Registry { key: &'static str },
    /// check macos configuration
    MacosPreference {
        domain: &'static str,
        key: &'static str,
    },
}

#[derive(Debug)]
pub enum Assertion {
    True,
    False,
    Equals(Value),
    NotEquals(Value),
    Contains(&'static str),
    GreaterThan(i64),
    LessThan(i64),
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
pub enum CheckStatus {
    Pass,
    Fail,
    Unknown,
    NotApplicable,
    Error,
}

#[derive(Debug)]
pub struct CheckResult {
    pub status: CheckStatus,
    pub value: Option<Value>,
}

pub fn evaluate(check: &Check, system: &dyn System) -> CheckResult {
    let value = match observe(&check.query, system) {
        Ok(value) => value,
        Err(_) => {
            return CheckResult {
                status: CheckStatus::Error,
                value: None,
            };
        }
    };

    let status = evaluate_assertion(&check.assertion, &value);

    CheckResult {
        status,
        value: Some(value),
    }
}

fn observe(query: &Query, system: &dyn System) -> std::io::Result<Value> {
    match query {
        Query::FileExists { path } => Ok(Value::Boolean(
            system.path_exists(std::path::Path::new(path))?,
        )),

        Query::Command { program, args } => {
            let result = system.command(program, args)?;

            Ok(Value::String(
                String::from_utf8_lossy(&result.stdout).into_owned(),
            ))
        }

        Query::Sysctl { key } => {
            // system-specific implementation
            todo!()
        }

        Query::Registry { key } => {
            // system-specific implementation
            todo!()
        }

        Query::MacosPreference { domain, key } => {
            // system-specific implementation
            todo!()
        }
    }
}

fn evaluate_assertion(assertion: &Assertion, value: &Value) -> CheckStatus {
    match assertion {
        Assertion::True => match value {
            Value::Boolean(true) => CheckStatus::Pass,
            Value::Boolean(false) => CheckStatus::Fail,
            _ => CheckStatus::Error,
        },

        Assertion::False => match value {
            Value::Boolean(false) => CheckStatus::Pass,
            Value::Boolean(true) => CheckStatus::Fail,
            _ => CheckStatus::Error,
        },

        Assertion::Equals(expected) => {
            if value == expected {
                CheckStatus::Pass
            } else {
                CheckStatus::Fail
            }
        }

        // ...
        _ => todo!(),
    }
}
