use subprocess::NullFile;

use crate::{log_fail, log_info, log_ok};
use crate::args::{ARGS, OPTIONS};
use crate::util::dpkg;
use crate::util::verification::os_verifier;

pub fn lock() {
    os_verifier();
    karen::escalate_if_needed().unwrap();
    
    log_info!("Locking apt ...");

    let packages = dpkg::get_packages();
    
    let exit_status = if OPTIONS.verbose {
        subprocess::Exec::shell(format!("apt-mark hold {}", packages.join(" ")))
            .join()
            .expect("Failed to execute command")
    } else {
        subprocess::Exec::shell(format!("apt-mark hold {} &> /dev/null", packages.join(" ")))
            .stdout(NullFile)
            .stderr(NullFile)
            .join()
            .expect("Failed to execute command")
    }; 

    if exit_status.success() {
        log_ok!("Finished apt locking");
    } else {
        log_fail!("Command execution failed");
    }
}