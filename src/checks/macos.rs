use crate::checks::{
    Check,
    macos::{
        docker_rootless::DOCKER_ROOTLESS, firewall_enabled::FIREWALL_ENABLED,
        homebrew_updated::HOMEBREW_UPDATED, sip_enabled::SIP_ENABLED,
    },
};

pub mod docker_rootless;
pub mod firewall_enabled;
pub mod homebrew_updated;
pub mod sip_enabled;

pub static CHECKS: &[Check] = &[
    FIREWALL_ENABLED,
    DOCKER_ROOTLESS,
    SIP_ENABLED,
    HOMEBREW_UPDATED,
];
