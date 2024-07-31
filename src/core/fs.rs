pub fn mount_system() {
    subprocess::Exec::shell("sh -c 'mount --bind /dev /system/dev'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'mount --bind /sys /system/sys'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'mount --bind /proc /system/proc'").popen().unwrap();
}

pub fn umount_system() {
    subprocess::Exec::shell("sh -c 'umount /system/dev'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'umount /system/sys'").popen().unwrap();
    subprocess::Exec::shell("sh -c 'umount /system/proc'").popen().unwrap();
}