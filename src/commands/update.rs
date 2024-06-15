use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount, remount, MountType};
use crate::args::OPTIONS;
use crate::util::verification::{os_verifier, root_verifier};
use crate::log_info;

pub fn update() {
    os_verifier();
    root_verifier();

    log_info!("Starting os update ...");

    remount(&MountType::ReadWrite);
    mount();

    if OPTIONS.yes {
        subprocess::Exec::shell("/usr/sbin/chroot /system sh -c 'apt update && apt -y upgrade'").popen().unwrap();
    }
    else {
        subprocess::Exec::shell("/usr/sbin/chroot /system sh -c 'apt update && apt upgrade'").popen().unwrap();
    }


    lock();
    remount(&MountType::ReadOnly);
    umount();

    log_info!("Finished os update ...");
}