use std::fs;
use std::path::Path;
use crate::log_fail;

pub fn os_verifier() {
    if !Path::new("/user").exists() && !Path::new("/system").exists() {
        log_fail!("somgr can only be executed on a StrawberryOS system");
        std::process::exit(1);
    }
}

pub fn is_chroot() -> bool {
    let root_symlink = fs::read_link("/proc/1/root").unwrap_or_else(|_| Path::new("/").to_path_buf());
    let root_path = fs::canonicalize("/").unwrap_or_else(|_| Path::new("/").to_path_buf());

    root_symlink != root_path
}