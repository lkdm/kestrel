use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

use crate::system::{System, SystemError};

#[cfg(target_os = "macos")]
#[path = "host/macos.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "host/linux.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "host/windows.rs"]
mod platform;

#[derive(Debug, Error)]
pub enum HostError {
    #[error("system error: {0}")]
    System(#[from] SystemError),
}

pub type Result<T> = std::result::Result<T, HostError>;

pub struct HostInfo {
    pub platform_uuid: Uuid,
    pub hostname: String,
    pub home_dir: PathBuf,
    pub is_root: bool,
}

impl HostInfo {
    pub fn collect(system: &dyn System) -> Result<Self> {
        Ok(Self {
            platform_uuid: todo!(),
            hostname: todo!(),
            home_dir: todo!(),
            is_root: todo!(),
        })
    }
}
