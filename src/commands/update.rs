use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount, remount, MountType};
use crate::log_info;

pub fn update() {
    log_info!("Starting os update ...");
    
    remount(&MountType::ReadWrite);
    mount();
    subprocess::Exec::shell("/usr/sbin/chroot /system sh -c 'apt update && apt -q upgrade'").popen().unwrap();

    lock();
    remount(&MountType::ReadOnly);
    umount();
    
    log_info!("Finished os update ...");
}