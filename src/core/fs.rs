use std::fs::File;
use std::os::fd::AsRawFd;
use nix::fcntl::{PosixFadviseAdvice, posix_fadvise};
use nix::unistd::{Whence, lseek};

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

pub fn fadvise(path: impl ToString) {
    let file = File::open(path.to_string()).unwrap();
    let file_len = lseek(file.as_raw_fd(), 0, Whence::SeekEnd).unwrap();

    posix_fadvise(file.as_raw_fd(), 0, file_len, PosixFadviseAdvice::POSIX_FADV_DONTNEED).unwrap();
}