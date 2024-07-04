use crate::log_info;
use crate::util::verification::os_verifier;

pub fn run(args: &[String]) {
    os_verifier();
    karen::escalate_if_needed().unwrap();
    
    let command = args.get(1..).unwrap().join(" ");
    
    log_info!(format!("Executing command '{command}'"));
    subprocess::Exec::shell(format!("/usr/sbin/chroot /system {command}")).popen().unwrap();
}
