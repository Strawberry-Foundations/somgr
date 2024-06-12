use crate::log_info;
use crate::util::fs;

pub fn shell() {
    log_info!("Mounting file systems ...");
    fs::mount_system();
    
    log_info!("Entering chroot ...");
    subprocess::Exec::shell("sh -c '/usr/sbin/chroot /system'").popen().unwrap();

    log_info!("Unmounting file systems ...");
    fs::umount_system();
    
    log_info!("Leaving chroot ...");
}
