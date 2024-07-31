use crate::{log_info, log_ok, log_panic};
use crate::core::subprocess::subprocess;
use crate::util::dpkg;
use crate::util::verification::os_verifier;

pub fn lock() {
    os_verifier();
    karen::escalate_if_needed().unwrap();
    
    log_info!("Locking apt ...");

    let packages = dpkg::get_packages();

    let exit_status = subprocess(format!("apt-mark hold {}", packages.join(" ")));

    if exit_status.success() {
        log_ok!("Finished apt locking");
    } else {
        log_panic!("Command execution failed");
    }
}