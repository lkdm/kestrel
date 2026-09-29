use std::path::PathBuf;

use crate::{
    check_id,
    checks::{Check, CheckReturnedResult},
    system::{System, common::command::CommandRequest},
};

pub static SSH_KEYS_STRONG: Check = Check {
    id: check_id!("842F7AB0-29F6-4FE0-8500-CE2ABB9F7F2F"),
    name: "ssh-keys-strong",
    description: "SSH private keys use sufficiently strong cryptographic algorithms and key sizes.",
    recommendation: "Use RSA keys of at least 2048 bits (4096 recommended), ECDSA P-521, or Ed25519. Do not use DSA keys.",
    run: ssh_keys_strong,
};

fn ssh_keys_strong(system: &dyn System) -> CheckReturnedResult {
    let home = std::env::var("HOME").unwrap_or_default();
    let ssh_dir = PathBuf::from(home).join(".ssh");

    let files = system.read_directory(&ssh_dir)?;

    for path in files {
        if !path.is_file() || path.extension().is_some_and(|ext| ext == "pub") {
            continue;
        }

        let path = match path.to_str() {
            Some(path) => path,
            None => continue,
        };

        // ssh-keygen -yf extracts the public key from a private key.
        // This lets us distinguish private keys from other files in ~/.ssh.
        let result =
            match system.command(&CommandRequest::new("/usr/bin/ssh-keygen", &["-yf", path])) {
                Ok(result) => result,
                Err(_) => continue,
            };

        let public_key = String::from_utf8_lossy(&result.stdout);
        let key_type = match public_key.split_whitespace().next() {
            Some(key_type) => key_type,
            None => continue,
        };

        let result = system.command(&CommandRequest::new("/usr/bin/ssh-keygen", &["-lf", path]))?;

        let output = String::from_utf8_lossy(&result.stdout);
        let bits = match output
            .split_whitespace()
            .next()
            .and_then(|value| value.parse::<u32>().ok())
        {
            Some(bits) => bits,
            None => return Ok(false),
        };

        let strong = match key_type {
            "ssh-rsa" => bits >= 2048,
            "ssh-dss" => false,
            "ecdsa-sha2-nistp521" => bits >= 521,
            "ssh-ed25519" => bits >= 256,
            _ => false,
        };

        if !strong {
            return Ok(false);
        }
    }

    Ok(true)
}
