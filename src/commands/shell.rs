use stblib::colors::{BOLD, C_RESET, GREEN, RESET};

pub fn shell() {
    println!("{BOLD}{GREEN}=>{RESET} Entering chroot ...{C_RESET}");
    subprocess::Exec::shell("sh -c '/usr/sbin/chroot /system'").popen().unwrap();
    println!("{BOLD}{GREEN}=>{RESET} Leaving chroot ...{C_RESET}");
}
