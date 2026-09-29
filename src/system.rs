use std::path::{Path, PathBuf};

use crate::system::common::command::{CommandError, CommandOutput, CommandRequest};
pub mod common;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::MacOSSystem as SystemImpl;

use thiserror::Error;
#[cfg(target_os = "windows")]
pub use windows::WindowsSystem as SystemImpl;

#[cfg(target_os = "linux")]
pub use linux::LinuxSystem as SystemImpl;

#[cfg(test)]
pub mod test;

#[derive(Debug, Error)]
pub enum SystemError {
    #[error(transparent)]
    Command(#[from] CommandError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SystemError>;

/// represents host system
pub trait System: Send + Sync {
    /// run a system command
    fn command(&self, request: &CommandRequest) -> Result<CommandOutput>;

    /// check if path exists
    fn path_exists(&self, path: &Path) -> Result<bool>;

    /// list files in a directory
    fn read_directory(&self, path: &Path) -> Result<Vec<PathBuf>>;

    /// read a file, returning binary data
    fn read_binary(&self, path: &Path) -> Result<Vec<u8>>;
}

pub trait CommandResultExt {
    fn ensure_success(self) -> Result<CommandOutput>;
    fn ignore_not_found(self) -> Result<Option<CommandOutput>>;
}

impl CommandResultExt for Result<CommandOutput> {
    /// Creates an error if the program did not return with a success exit code
    fn ensure_success(self) -> Result<CommandOutput> {
        let output = self?;

        if output.success() {
            Ok(output)
        } else {
            Err(CommandError::NonZeroExit {
                program: output.program.clone(),
                code: output.status.code(),
                stderr: output.stderr_utf8().into_owned(),
            }
            .into())
        }
    }

    /// Ignores an error caused by the program not being found
    fn ignore_not_found(self) -> Result<Option<CommandOutput>> {
        match self {
            Ok(output) => Ok(Some(output)),
            Err(SystemError::Command(CommandError::NotFound { .. })) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
