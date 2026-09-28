use std::borrow::Cow;
use std::process::{Command, ExitStatus};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("failed to spawn `{program}`")]
    Spawn {
        program: String,
        #[source]
        source: std::io::Error,
    },

    #[error("`{program}` exited with a non-zero status")]
    NonZeroExit {
        program: String,
        code: Option<i32>,
        stderr: String,
    },
}

pub type Result<T> = std::result::Result<T, CommandError>;

/// What to run. One value: built once, reused, and doubles as the
/// lookup key when mocking with `TestSystem`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CommandRequest {
    pub program: String,
    pub args: Vec<String>,
}

impl CommandRequest {
    pub fn new(program: impl Into<String>, args: &[&str]) -> Self {
        Self {
            program: program.into(),
            args: args.iter().map(|a| (*a).to_owned()).collect(),
        }
    }

    pub fn run(&self) -> Result<CommandOutput> {
        let output = Command::new(&self.program)
            .args(&self.args)
            .output()
            .map_err(|source| CommandError::Spawn {
                program: self.program.clone(),
                source,
            })?;

        Ok(CommandOutput {
            program: self.program.clone(),
            args: self.args.clone(),
            status: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub program: String,
    pub args: Vec<String>,
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl CommandOutput {
    pub fn success(&self) -> bool {
        self.status.success()
    }

    /// Convert stdout to utf-8
    pub fn stdout_utf8(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.stdout)
    }

    /// Convert stderr to utf-8
    pub fn stderr_utf8(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.stderr)
    }
}

#[cfg(test)]
impl CommandRequest {
    pub fn success(&self, stdout: impl Into<Vec<u8>>) -> (CommandRequest, CommandOutput) {
        (self.clone(), CommandOutput::test_success(stdout))
    }

    pub fn failure(&self, stdout: impl Into<Vec<u8>>) -> (CommandRequest, CommandOutput) {
        (self.clone(), CommandOutput::test_failure(stdout))
    }
}

#[cfg(test)]
impl CommandOutput {
    pub fn test_success(stdout: impl Into<Vec<u8>>) -> Self {
        Self {
            program: String::new(),
            args: Vec::new(),
            status: test_exit_status(0),
            stdout: stdout.into(),
            stderr: Vec::new(),
        }
    }

    pub fn test_failure(stdout: impl Into<Vec<u8>>) -> Self {
        Self {
            program: String::new(),
            args: Vec::new(),
            status: test_exit_status(1),
            stdout: stdout.into(),
            stderr: Vec::new(),
        }
    }
}

#[cfg(test)]
fn test_exit_status(code: i32) -> ExitStatus {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        ExitStatus::from_raw(code << 8)
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::ExitStatusExt;
        ExitStatus::from_raw(code as u32)
    }
}
