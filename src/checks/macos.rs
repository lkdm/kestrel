use crate::checks::{
    Check,
    macos::{
        docker_rootless::DOCKER_ROOTLESS, filevault_enabled::FILEVAULT_ENABLED,
        firewall_block_all::FIREWALL_BLOCK_ALL, firewall_enabled::FIREWALL_ENABLED,
        gatekeeper_enabled::GATEKEEPER_ENABLED, homebrew_updated::HOMEBREW_UPDATED,
        macos_updated::MACOS_UPDATED, remote_login_disabled::REMOTE_LOGIN_DISABLED,
        sip_enabled::SIP_ENABLED,
    },
};

pub mod docker_rootless;
pub mod filevault_enabled;
pub mod firewall_block_all;
pub mod firewall_enabled;
pub mod gatekeeper_enabled;
pub mod homebrew_updated;
pub mod macos_updated;
pub mod remote_login_disabled;
pub mod sip_enabled;

pub static CHECKS: &[Check] = &[
    FIREWALL_ENABLED,
    DOCKER_ROOTLESS,
    SIP_ENABLED,
    HOMEBREW_UPDATED,
    MACOS_UPDATED,
    REMOTE_LOGIN_DISABLED,
    GATEKEEPER_ENABLED,
    FILEVAULT_ENABLED,
    FIREWALL_BLOCK_ALL,
];
