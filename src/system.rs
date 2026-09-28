use std::{
    io,
    path::{Path, PathBuf},
};

use crate::system::common::command::CommandResult;
pub mod common;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::MacOSSystem as SystemImpl;

#[cfg(target_os = "windows")]
pub use windows::WindowsSystem as SystemImpl;

#[cfg(target_os = "linux")]
pub use linux::LinuxSystem as SystemImpl;

#[cfg(test)]
pub mod test;

/// represents interaction with the host system
pub trait System: Send + Sync {
    /// run a system command
    fn command(&self, program: &str, args: &[&str]) -> io::Result<CommandResult>;

    /// check if path exists
    fn path_exists(&self, path: &Path) -> io::Result<bool>;

    /// list files in a directory
    fn read_directory(&self, path: &Path) -> io::Result<Vec<PathBuf>>;

    /// read a file, returning binary data
    fn read_binary(&self, path: &Path) -> io::Result<Vec<u8>>;
}
