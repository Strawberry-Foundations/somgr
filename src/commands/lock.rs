use std::fs::File;
use std::io::BufRead;

use subprocess::NullFile;
use stblib::colors::{BOLD, C_RESET, GREEN, RESET};

pub fn lock() {
    println!("{BOLD}{GREEN}=>{RESET} Locking apt ...{C_RESET}");
    subprocess::Exec::shell(
        "/usr/sbin/chroot / dpkg --get-selections | grep -v deinstall | awk '{print $1}' > /var/cache/apt/package_list.txt"
    ).popen().unwrap();

    let file = File::open("/var/cache/apt/package_list.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut packages = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            packages.push(word.to_string());
        }
    }

    let exit_status = subprocess::Exec::shell(format!("apt-mark hold {} &> /dev/null", packages.join(" ")))
        .stdout(NullFile)
        .stderr(NullFile)
        .join()
        .expect("Failed to execute command");

    if exit_status.success() {
        println!("{BOLD}{GREEN}=>{RESET} Finished apt locking ...{C_RESET}");
    } else {
        eprintln!("Command execution failed");
    }
}