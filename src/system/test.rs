use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::system::{Result, SystemError, common::command::CommandRequest};

use super::{CommandOutput, System};

#[derive(Default)]
pub struct TestSystem {
    commands: HashMap<CommandRequest, CommandOutput>,
    paths: HashMap<PathBuf, bool>,
    directories: HashMap<PathBuf, Vec<PathBuf>>,
    binaries: HashMap<PathBuf, Vec<u8>>,
}

impl TestSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_command(mut self, (request, output): (CommandRequest, CommandOutput)) -> Self {
        let previous = self.commands.insert(request, output);
        assert!(previous.is_none(), "command registered twice in TestSystem");
        self
    }

    pub fn with_path_exists(mut self, path: impl Into<PathBuf>, exists: bool) -> Self {
        self.paths.insert(path.into(), exists);
        self
    }

    pub fn with_directory(mut self, path: impl Into<PathBuf>, entries: Vec<PathBuf>) -> Self {
        self.directories.insert(path.into(), entries);
        self
    }

    pub fn with_binary(mut self, path: impl Into<PathBuf>, contents: Vec<u8>) -> Self {
        self.binaries.insert(path.into(), contents);
        self
    }
}

impl System for TestSystem {
    fn command(&self, request: &CommandRequest) -> Result<CommandOutput> {
        self.commands
            .get(request)
            .map(|output| CommandOutput {
                program: request.program.clone(),
                args: request.args.clone(),
                status: output.status,
                stdout: output.stdout.clone(),
                stderr: output.stderr.clone(),
            })
            .ok_or_else(|| {
                not_configured(&format!("command: {} {:?}", request.program, request.args))
            })
    }

    fn path_exists(&self, path: &Path) -> Result<bool> {
        self.paths
            .get(path)
            .copied()
            .ok_or_else(|| not_configured(&format!("path: {}", path.display())))
    }

    fn read_directory(&self, path: &Path) -> Result<Vec<PathBuf>> {
        self.directories
            .get(path)
            .cloned()
            .ok_or_else(|| not_configured(&format!("directory: {}", path.display())))
    }

    fn read_binary(&self, path: &Path) -> Result<Vec<u8>> {
        self.binaries
            .get(path)
            .cloned()
            .ok_or_else(|| not_configured(&format!("binary: {}", path.display())))
    }
}

fn not_configured(what: &str) -> SystemError {
    std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("test {what} not configured"),
    )
    .into()
}
