use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount};
use crate::log_info;

pub fn update() {
    log_info!("Starting os update ...");
    
    mount();
    subprocess::Exec::shell("/usr/sbin/chroot / apt update && apt -q upgrade").popen().unwrap();

    lock();
    umount();
    
    log_info!("Finished os update ...");
}