use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount, remount};
use crate::log_info;

pub fn update() {
    log_info!("Starting os update ...");
    
    remount();
    mount();
    subprocess::Exec::shell("/usr/sbin/chroot /system apt update && apt -q upgrade").popen().unwrap();

    lock();
    umount();
    
    log_info!("Finished os update ...");
}