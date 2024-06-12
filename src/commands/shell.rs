use crate::log_info;

pub fn shell() {
    log_info!("Mounting file systems ...");
    subprocess::Exec::shell("sh -c 'mount --bind /dev /system/dev'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'mount --bind /sys /system/sys'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'mount --bind /proc /system/proc'").popen().unwrap();
    
    log_info!("Entering chroot ...");
    subprocess::Exec::shell("sh -c '/usr/sbin/chroot /system'").popen().unwrap();

    log_info!("Unmounting file systems ...");
    subprocess::Exec::shell("sh -c 'umount /system/dev'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'umount /system/sys'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'umount /system/proc'").popen().unwrap();
    
    log_info!("Leaving chroot ...");
}
