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
