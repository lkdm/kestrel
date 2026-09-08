pub struct Check {
    pub id: &'static str,
    pub description: &'static str,
    pub query: Query,
    pub assertion: Assertion,
}

pub enum Query {
    FileExists {
        path: &'static str,
    },
    Command {
        program: &'static str,
        args: &'static [&'static str],
    },
    Sysctl {
        key: &'static str,
    },
    Registry {
        key: &'static str,
    },
    MacosPreference {
        domain: &'static str,
        key: &'static str,
    },
}

pub enum Assertion {
    Exists,
    Equals(Value),
    NotEquals(Value),
    Contains(Value),
    GreaterThan(i64),
    LessThan(i64),
}
