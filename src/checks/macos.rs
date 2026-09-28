use crate::checks::{
    Check,
    macos::{
        automatic_login_off::AUTOMATIC_LOGIN_DISABLED, docker_rootless::DOCKER_ROOTLESS,
        filevault_enabled::FILEVAULT_ENABLED, firewall_block_all::FIREWALL_BLOCK_ALL,
        firewall_enabled::FIREWALL_ENABLED, gatekeeper_enabled::GATEKEEPER_ENABLED,
        homebrew_updated::HOMEBREW_UPDATED, macos_updated::MACOS_UPDATED,
        password_after_inactivity::PASSWORD_AFTER_INACTIVITY,
        remote_login_disabled::REMOTE_LOGIN_DISABLED, sip_enabled::SIP_ENABLED,
        std_user::STANDARD_USER,
    },
};

pub mod automatic_login_off;
pub mod docker_rootless;
pub mod filevault_enabled;
pub mod firewall_block_all;
pub mod firewall_enabled;
pub mod gatekeeper_enabled;
pub mod homebrew_updated;
pub mod macos_updated;
pub mod password_after_inactivity;
pub mod remote_login_disabled;
pub mod sip_enabled;
pub mod std_user;

pub static CHECKS: &[Check] = &[
    PASSWORD_AFTER_INACTIVITY,
    STANDARD_USER,
    AUTOMATIC_LOGIN_DISABLED,
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
