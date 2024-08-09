use std::fs::{File};
use std::io::{BufRead, BufReader};

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

pub fn get_package_entry(package_name: &str, status_file: &str) -> Option<String> {
    let file = File::open(status_file).unwrap();
    let reader = BufReader::new(file);
    let mut package_entry = String::new();
    let mut found_package = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            if found_package {
                return Some(package_entry);
            }
            package_entry.clear();
            found_package = false;
        } else {
            if line.starts_with(&format!("Package: {}", package_name)) {
                found_package = true;
            }
            if found_package {
                package_entry.push_str(&line);
                package_entry.push('\n');
            }
        }
    }
    None
}

pub fn get_package_version(package_name: &str, status_file: &str) -> Option<String> {
    let file = File::open(status_file).unwrap();
    let reader = BufReader::new(file);
    let mut package_entry = String::new();
    let mut found_package = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            if found_package {
                if let Some(version) = package_entry.lines().find_map(|line| {
                    if line.starts_with("Version: ") {
                        line.split_whitespace().nth(1).map(String::from)
                    } else {
                        None
                    }
                }) {
                    return Some(version);
                }
            }
            package_entry.clear();
            found_package = false;
        } else {
            if line.starts_with(&format!("Package: {}", package_name)) {
                found_package = true;
            }
            if found_package {
                package_entry.push_str(&line);
                package_entry.push('\n');
            }
        }
    }
    None
}

pub fn update_version_in_entry(entry: &str, new_version: &str) -> String {
    entry.lines().map(|line| {
        if line.starts_with("Version: ") {
            format!("Version: {}", new_version)
        } else {
            line.to_string()
        }
    }).collect::<Vec<_>>().join("\n")
}

pub fn get_package_version_from_entry(entry: &str) -> Option<String> {
    entry.lines().find_map(|line| {
        if line.starts_with("Version: ") {
            line.split_whitespace().nth(1).map(String::from)
        } else {
            None
        }
    })
}

