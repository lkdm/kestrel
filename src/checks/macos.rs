use crate::checks::{
    Check,
    macos::{
        admin_password_for_preferences::ADMIN_PASSWORD_FOR_PREFERENCES,
        airdrop_secured::AIRDROP_SECURED, automatic_login_off::AUTOMATIC_LOGIN_DISABLED,
        filevault_enabled::FILEVAULT_ENABLED, firewall_block_all::FIREWALL_BLOCK_ALL,
        firewall_enabled::FIREWALL_ENABLED, gatekeeper_enabled::GATEKEEPER_ENABLED,
        macos_updated::MACOS_UPDATED, password_after_inactivity::PASSWORD_AFTER_INACTIVITY,
        printer_sharing_off::PRINTER_SHARING_DISABLED,
        remote_login_disabled::REMOTE_LOGIN_DISABLED, sip_enabled::SIP_ENABLED,
        std_user::STANDARD_USER, wifi_encrypted::WIFI_ENCRYPTED,
    },
    shared::{
        docker_rootless::DOCKER_ROOTLESS, homebrew_updated::HOMEBREW_UPDATED,
        ssh_key_encryption::SSH_KEYS_STRONG,
    },
};

pub mod admin_password_for_preferences;
pub mod airdrop_secured;
pub mod automatic_login_off;
pub mod filevault_enabled;
pub mod firewall_block_all;
pub mod firewall_enabled;
pub mod gatekeeper_enabled;
pub mod macos_updated;
pub mod password_after_inactivity;
pub mod printer_sharing_off;
pub mod remote_login_disabled;
pub mod sip_enabled;
pub mod std_user;
pub mod wifi_encrypted;

pub static CHECKS: &[Check] = &[
    // macOS
    ADMIN_PASSWORD_FOR_PREFERENCES,
    PASSWORD_AFTER_INACTIVITY,
    STANDARD_USER,
    AUTOMATIC_LOGIN_DISABLED,
    FIREWALL_ENABLED,
    SIP_ENABLED,
    MACOS_UPDATED,
    REMOTE_LOGIN_DISABLED,
    GATEKEEPER_ENABLED,
    FILEVAULT_ENABLED,
    FIREWALL_BLOCK_ALL,
    WIFI_ENCRYPTED,
    PRINTER_SHARING_DISABLED,
    AIRDROP_SECURED,
    // Shared
    SSH_KEYS_STRONG,
    DOCKER_ROOTLESS,
    HOMEBREW_UPDATED,
];
