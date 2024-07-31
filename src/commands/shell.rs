use crate::core::subprocess::popen;
use crate::core::fs;
use crate::util::verification::os_verifier;
use crate::log_info;

pub fn shell() {
    os_verifier();
    karen::escalate_if_needed().unwrap();
    
    log_info!("Mounting file systems ...");
    fs::mount_system();
    
    log_info!("Entering chroot ...");
    popen("sh -c '/usr/sbin/chroot /system'");

    log_info!("Unmounting file systems ...");
    fs::umount_system();
    
    log_info!("Leaving chroot ...");
}
