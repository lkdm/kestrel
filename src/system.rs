use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{checks::Value, system::common::command::CommandResult};
pub mod common;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::MacOSSystem as SystemImpl;

#[cfg(target_os = "windows")]
pub use windows::WindowsSystem as SystemImpl;

#[cfg(target_os = "linux")]
pub use linux::LinuxSystem as SystemImpl;

/// represents interaction with the host system
pub trait System {
    /// run a system command
    fn command(&self, program: &str, args: &[&str]) -> io::Result<CommandResult>;

    /// check if path exists
    fn path_exists(&self, path: &Path) -> io::Result<bool>;

    /// list files in a directory
    fn read_directory(&self, path: &Path) -> io::Result<Vec<PathBuf>>;

    /// read a file, returning binary data
    fn read_binary(&self, path: &Path) -> io::Result<Vec<u8>>;

    /// reads a system property
    ///
    /// returns `None` if the check is not applicable
    fn read_property(&self, property: &SystemProperty) -> Option<io::Result<Value>>;
}

pub enum SystemProperty {
    FirewallEnabled,
    DiskEncryptionEnabled,
    ScreenLockTimeout,
    WindowsDefenderEnabled,
}
