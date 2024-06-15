use std::path::Path;
use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount, remount, MountType};
use crate::{log_fail, log_info};

pub fn update() {
    if !Path::new("/user").exists() {
        log_fail!("somgr can only be executed on a StrawberryOS system");
        std::process::exit(1);
    }
    
    log_info!("Starting os update ...");
    
    remount(&MountType::ReadWrite);
    mount();
    subprocess::Exec::shell("/usr/sbin/chroot /system sh -c 'apt update && apt upgrade'").popen().unwrap();

    lock();
    remount(&MountType::ReadOnly);
    umount();
    
    log_info!("Finished os update ...");
}