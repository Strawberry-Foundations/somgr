use std::fs::{File};
use std::io::{BufRead, BufReader};

use regex::Regex;

pub fn get_packages() -> Vec<String> {
    subprocess::Exec::shell(
        "/usr/sbin/chroot /system dpkg --get-selections | grep -v deinstall | awk '{print $1}' > /var/cache/apt/package_list.txt"
    ).popen().unwrap();

    let file = File::open("/var/cache/apt/package_list.txt").unwrap();
    let reader = BufReader::new(file);

    let mut packages = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            packages.push(word.to_string());
        }
    }

    packages
}

pub fn get_package_version(package_name: &str, status_file: &str) -> Option<String> {
    let file = File::open(status_file).ok()?;
    let reader = BufReader::new(file);
    let mut found = false;
    for line in reader.lines() {
        let line = line.ok()?;
        if line.starts_with("Package: ") && line.split_whitespace().nth(1) == Some(package_name) {
            found = true;
        }
        if found && line.starts_with("Version: ") {
            return line.split_whitespace().nth(1).map(String::from);
        }
    }
    None
}

pub fn update_version_in_entry(entry: &str, new_version: &str) -> String {
    let re = Regex::new(r"Version: .+").unwrap();
    re.replace(entry, format!("Version: {}", new_version).as_str()).to_string()
}