use crate::{
    checks::{Check, CheckReturnedResult},
    system::System,
};

fn standard_user(system: &dyn System) -> CheckReturnedResult {
    let result = system.command("/usr/bin/id", &["-Gn"])?;

    let output = String::from_utf8_lossy(&result.stdout);

    Ok(!output.split_whitespace().any(|group| group == "admin"))
}

pub static STANDARD_USER: Check = Check {
    id: "macos-standard-user",
    description: "The daily user account is a Standard user rather than an Administrator.",
    recommendation: "Use a Standard user account for daily activities instead of an Administrator account.",
    run: standard_user,
};
