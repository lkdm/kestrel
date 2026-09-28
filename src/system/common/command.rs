use std::process::ExitStatus;

pub struct CommandResult {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl CommandResult {
    pub fn success(&self) -> bool {
        self.status.success()
    }
}

impl CommandResult {
    #[cfg(test)]
    pub fn test_success(stdout: impl Into<Vec<u8>>) -> Self {
        Self {
            status: test_exit_status(true),
            stdout: stdout.into(),
            stderr: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn test_failure(stdout: impl Into<Vec<u8>>) -> Self {
        Self {
            status: test_exit_status(false),
            stdout: stdout.into(),
            stderr: Vec::new(),
        }
    }
}

#[cfg(test)]
fn test_exit_status(success: bool) -> ExitStatus {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;

        if success {
            ExitStatus::from_raw(0)
        } else {
            ExitStatus::from_raw(1 << 8)
        }
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::ExitStatusExt;

        ExitStatus::from_raw(if success { 0 } else { 1 })
    }
}
