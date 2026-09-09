use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{checks::Value, system::SystemProperty};

use super::{CommandResult, System, common};

#[derive(Debug, Default)]
pub struct MacOSSystem;

impl System for MacOSSystem {
    fn command(&self, program: &str, args: &[&str]) -> io::Result<CommandResult> {
        common::command(program, args)
    }

    fn path_exists(&self, path: &Path) -> io::Result<bool> {
        common::path_exists(path)
    }

    fn read_directory(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        common::read_directory(path)
    }

    fn read_binary(&self, path: &Path) -> io::Result<Vec<u8>> {
        common::read_binary(path)
    }

    fn read_property(&self, property: &SystemProperty) -> Option<io::Result<Value>> {
        match property {
            SystemProperty::FirewallEnabled => Some(self.firewall_enabled()),

            SystemProperty::DiskEncryptionEnabled => Some(self.disk_encryption_enabled()),

            SystemProperty::ScreenLockTimeout => Some(self.screen_lock_timeout()),

            // This property has no meaningful macOS implementation.
            SystemProperty::WindowsDefenderEnabled => None,
        }
    }
}

impl MacOSSystem {
    fn firewall_enabled(&self) -> io::Result<Value> {
        let result = self.command(
            "/usr/libexec/ApplicationFirewall/socketfilterfw",
            &["--getglobalstate"],
        )?;

        let output = String::from_utf8_lossy(&result.stdout);

        Ok(Value::Boolean(output.contains("Firewall is enabled")))
    }

    fn disk_encryption_enabled(&self) -> io::Result<Value> {
        let result = self.command("fdesetup", &["status"])?;

        let output = String::from_utf8_lossy(&result.stdout);

        Ok(Value::Boolean(output.contains("FileVault is On")))
    }

    fn screen_lock_timeout(&self) -> io::Result<Value> {
        // Implement when you actually need this property.
        todo!()
    }
}
