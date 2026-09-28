use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use crate::checks::{Check, CheckReturnedResult};

use super::{CommandResult, System};

#[derive(Default)]
pub struct TestSystem {
    commands: HashMap<CommandKey, CommandResult>,
    paths: HashMap<PathBuf, bool>,
    directories: HashMap<PathBuf, Vec<PathBuf>>,
    binaries: HashMap<PathBuf, Vec<u8>>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct CommandKey {
    program: String,
    args: Vec<String>,
}

impl TestSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(
        mut self,
        program: impl Into<String>,
        args: &[&str],
        result: CommandResult,
    ) -> Self {
        let key = CommandKey {
            program: program.into(),
            args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        };

        self.commands.insert(key, result);
        self
    }

    pub fn path_exists(mut self, path: impl Into<PathBuf>, exists: bool) -> Self {
        self.paths.insert(path.into(), exists);
        self
    }

    pub fn directory(mut self, path: impl Into<PathBuf>, entries: Vec<PathBuf>) -> Self {
        self.directories.insert(path.into(), entries);
        self
    }

    pub fn binary(mut self, path: impl Into<PathBuf>, contents: Vec<u8>) -> Self {
        self.binaries.insert(path.into(), contents);
        self
    }

    pub fn command_stdout(
        self,
        program: impl Into<String>,
        args: &[&str],
        stdout: impl Into<Vec<u8>>,
    ) -> Self {
        self.command(program, args, CommandResult::test_success(stdout))
    }
}

impl System for TestSystem {
    fn command(&self, program: &str, args: &[&str]) -> io::Result<CommandResult> {
        let key = CommandKey {
            program: program.to_owned(),
            args: args.iter().map(|arg| (*arg).to_owned()).collect(),
        };

        self.commands
            .get(&key)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("test command not configured: {} {:?}", program, args),
                )
            })
            .map(|result| CommandResult {
                status: result.status,
                stdout: result.stdout.clone(),
                stderr: result.stderr.clone(),
            })
    }

    fn path_exists(&self, path: &Path) -> io::Result<bool> {
        self.paths.get(path).copied().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("test path not configured: {}", path.display()),
            )
        })
    }

    fn read_directory(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        self.directories.get(path).cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("test directory not configured: {}", path.display()),
            )
        })
    }

    fn read_binary(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.binaries.get(path).cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("test binary not configured: {}", path.display()),
            )
        })
    }
}
