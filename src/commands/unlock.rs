use subprocess::NullFile;
use stblib::colors::{BOLD, C_RESET, GREEN, RESET};

use crate::util::dpkg;

pub fn unlock() {
    println!("{BOLD}{GREEN}=>{RESET} Unlocking apt (DANGEROUS!) ...{C_RESET}");

    let packages = dpkg::get_packages();

    let exit_status = subprocess::Exec::shell(format!("apt-mark unhold {} &> /dev/null", packages.join(" ")))
        .stdout(NullFile)
        .stderr(NullFile)
        .join()
        .expect("Failed to execute command");

    if exit_status.success() {
        println!("{BOLD}{GREEN}=>{RESET} Finished apt unlocking ...{C_RESET}");
    } else {
        eprintln!("Command execution failed");
    }
}