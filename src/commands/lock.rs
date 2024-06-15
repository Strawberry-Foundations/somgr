use subprocess::NullFile;
use crate::log_info;

use crate::util::dpkg;

pub fn lock() {
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
        eprintln!("Command execution failed");
    }
}