use crate::checks::{Check, macos::firewall_enabled::FIREWALL_ENABLED};

pub mod firewall_enabled;

pub static CHECKS: &[Check] = &[FIREWALL_ENABLED];
