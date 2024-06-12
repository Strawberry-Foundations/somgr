use crate::log_info;
use crate::util::fs::{mount_system, umount_system};

pub fn remount() {
    log_info!("Remounting /system in rw mode...");
    subprocess::Exec::shell("mount -o remount,rw /system").popen().unwrap();
}

pub fn mount() {
    log_info!("Binding /dev, /sys and /proc to /system ...");
    mount_system()
}

pub fn umount() {
    log_info!("Unmounting system bindings in /system ...");
    umount_system()
}
