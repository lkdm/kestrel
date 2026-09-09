use std::{
    io,
    path::{Path, PathBuf},
};

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
}
