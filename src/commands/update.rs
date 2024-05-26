use stblib::colors::{BOLD, C_RESET, GREEN, RESET};
use crate::commands::lock::lock;

pub fn update() {
    println!("{BOLD}{GREEN}=>{RESET} Starting os update ...{C_RESET}");
    subprocess::Exec::shell("/usr/sbin/chroot / apt update && apt -q upgrade").popen().unwrap();

    lock();
    
    println!("{BOLD}{GREEN}=>{RESET} Finished os update ...{C_RESET}");

}