pub mod command;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// check if the path exists
pub fn path_exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

/// read the directory
pub fn read_directory(path: &Path) -> io::Result<Vec<PathBuf>> {
    fs::read_dir(path)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect()
}

/// read a file in binary
pub fn read_binary(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path)
}
