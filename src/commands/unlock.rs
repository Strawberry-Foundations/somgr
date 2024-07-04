use subprocess::NullFile;

use crate::{log_fail, log_info};
use crate::util::dpkg;
use crate::util::verification::os_verifier;

pub fn unlock() {
    os_verifier();
    karen::escalate_if_needed().unwrap();
    
    log_info!("Unlocking apt (DANGEROUS!) ...");

    let packages = dpkg::get_packages();

    let exit_status = subprocess::Exec::shell(format!("apt-mark unhold {} &> /dev/null", packages.join(" ")))
        .stdout(NullFile)
        .stderr(NullFile)
        .join()
        .expect("Failed to execute command");

    if exit_status.success() {
        log_info!("Finished apt unlocking");
    } else {
        log_fail!("Command execution failed");
    }
}