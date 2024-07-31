use crate::core::subprocess::popen;
use crate::util::verification::os_verifier;
use crate::log_info;

pub fn run(args: &[String]) {
    os_verifier();
    karen::escalate_if_needed().unwrap();
    
    let command = args.get(1..).unwrap().join(" ");
    
    log_info!(format!("Executing command '{command}'"));
    popen(format!("/usr/sbin/chroot /system {command}"));
}
