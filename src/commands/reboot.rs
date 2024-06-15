use std::io::Write;
use std::process::Command;
use crate::{log_fail, log_info};
use crate::util::verification::root_verifier;

pub fn reboot_fw() {
    root_verifier();

    let output = Command::new("efibootmgr")
        .arg("--bootnext")
        .arg("0")
        .output()
        .unwrap_or_else(|_| {
            log_fail!("Failed to run command");
            std::process::exit(1);
        });

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr).unwrap_or_else(|_| {
            log_fail!("Failed to set bootnext");
            std::process::exit(1);
        });
    }
    
    log_info!("Rebooting system into firmware settings");
    
    let output = Command::new("reboot")
        .output()
        .unwrap_or_else(|_| {
            log_fail!("Failed to run command");
            std::process::exit(1);
        });

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr).unwrap_or_else(|_| {
            log_fail!("Failed to reboot");
            std::process::exit(1);
        });
    }
}