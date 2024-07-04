use std::path::Path;
use crate::log_fail;

pub fn os_verifier() {
    if !Path::new("/user").exists() {
        log_fail!("somgr can only be executed on a StrawberryOS system");
        std::process::exit(1);
    }
}
