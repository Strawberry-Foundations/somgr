use crate::log_info;
use crate::util::fs::{mount_system, umount_system};

pub enum MountType {
    ReadWrite,
    ReadOnly
}

pub fn remount(mount_type: MountType) {
    match mount_type {
        MountType::ReadWrite => {
            log_info!("Remounting /system in rw mode...");
            subprocess::Exec::shell("mount -o remount,rw /system").popen().unwrap();
        }
        MountType::ReadOnly => {
            log_info!("Remounting /system in ro mode...");
            subprocess::Exec::shell("mount -o remount,ro /system").popen().unwrap();
        }
    }
}

pub fn mount() {
    log_info!("Binding /dev, /sys and /proc to /system ...");
    mount_system()
}

pub fn umount() {
    log_info!("Unmounting system bindings in /system ...");
    umount_system()
}
