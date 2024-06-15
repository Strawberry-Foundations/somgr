use std::path::Path;
use crate::log_fail;
use crate::util::permissions::check_root_permissions;

pub fn os_verifier() {
    if !Path::new("/user").exists() {
        log_fail!("somgr can only be executed on a StrawberryOS system");
        std::process::exit(1);
    }
}

pub fn root_verifier() {
    if !check_root_permissions() {
        log_fail!("You need to be root to use this command");
        std::process::exit(1);
    }
}