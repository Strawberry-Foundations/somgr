use crate::core::subprocess::popen;
use nix::{
    mount::{mount, umount},
    NixPath,
};
use std::env::set_current_dir;
use std::fs::create_dir_all;
use std::os::unix::fs;
use std::path::Path;

pub fn drop_fs_cache() {
    popen("sync");
    popen("echo 2 > /proc/sys/vm/drop_caches");
}

/// Mounts required filesystems for chrooting, returns a nix::Result<> that indicates if the mount process was successful
pub fn mount_system() -> nix::Result<()> {
    bind_mnt("/dev", &format!("/system/dev"))?;
    bind_mnt("/sys", &format!("/system/sys"))?;
    bind_mnt("/proc", &format!("/system/proc"))?;

    Ok(())
}

/// chroots into a directory, it is recommended to mount the /dev, /sys and /proc filesystems beforehand
pub fn chroot(new_root: &str) -> std::io::Result<()> {
    fs::chroot(new_root).expect("Unable to chroot");
    set_current_dir("/").expect("Unable to change current directory");
    Ok(())
}

pub fn umount_system() {
    umnt(format!("/system/dev").as_str()).expect("Unable to unmount /dev");
    umnt(format!("/system/sys").as_str()).expect("Unable to unmount /sys");
    umnt(format!("/system/proc").as_str()).expect("Unable to unmount /proc");
}

fn bind_mnt<P: ?Sized + NixPath>(source: &P, target: &P) -> nix::Result<()> {
    let flags = nix::mount::MsFlags::MS_BIND | nix::mount::MsFlags::MS_REC;

    mount(Some(source), target, None::<&str>, flags, None::<&str>)
}

fn umnt<P: ?Sized + NixPath>(target: &P) -> nix::Result<()> {
    umount(target)
}
