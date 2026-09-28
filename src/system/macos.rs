use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};

use crate::system::{self, Result, common::command::CommandRequest};

use super::{CommandOutput, System, common};

#[derive(Debug, Default)]
pub struct MacOSSystem;

impl System for MacOSSystem {
    fn command(&self, request: &CommandRequest) -> Result<CommandOutput> {
        Ok(request.run()?)
    }

    fn path_exists(&self, path: &Path) -> Result<bool> {
        Ok(common::path_exists(path)?)
    }

    fn read_directory(&self, path: &Path) -> Result<Vec<PathBuf>> {
        Ok(common::read_directory(path)?)
    }

    fn read_binary(&self, path: &Path) -> Result<Vec<u8>> {
        Ok(common::read_binary(path)?)
    }
}
