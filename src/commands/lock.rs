use subprocess::NullFile;

use crate::{log_fail, log_info};
use crate::util::dpkg;
use crate::util::verification::os_verifier;

pub fn lock() {
    os_verifier();
    log_info!("Locking apt ...");

    let packages = dpkg::get_packages();

    let exit_status = subprocess::Exec::shell(format!("apt-mark hold {} &> /dev/null", packages.join(" ")))
        .stdout(NullFile)
        .stderr(NullFile)
        .join()
        .expect("Failed to execute command");

    if exit_status.success() {
        log_info!("Finished apt locking");
    } else {
        log_fail!("Command execution failed");
    }
}