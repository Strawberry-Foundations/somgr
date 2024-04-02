use stblib::colors::{BOLD, C_RESET, GREEN, RESET};

pub fn remount() {
    println!("{BOLD}{GREEN}=>{RESET} Remounting /system in rw mode...{C_RESET}");
    subprocess::Exec::shell("mount -o remount,rw /system").popen().unwrap();
}
