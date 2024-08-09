use crate::core::subprocess::subprocess;
use crate::core::dpkg::status;
use crate::util::verification::os_verifier;
use crate::{log_info, log_ok, log_panic};

pub fn unlock() {
    os_verifier();
    karen::escalate_if_needed().unwrap();

    log_info!("Unlocking apt (DANGEROUS!) ...");

    let packages = status::get_packages();

    let exit_status = subprocess(format!("apt-mark unhold {}", packages.join(" ")));

    if exit_status.success() {
        log_ok!("Finished apt unlocking");
    } else {
        log_panic!("Command execution failed");
    }
}