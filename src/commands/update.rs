use std::{fs, io};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount, remount, MountType};
use crate::util::verification::{os_verifier, root_verifier};
use crate::statics::{DPKG_SYSTEM_STATUS, DPKG_USER_STATUS, DPKG_USER_STATUS_TMP};
use crate::args::OPTIONS;
use crate::log_info;
use crate::util::dpkg::{get_package_version, update_version_in_entry};

pub fn update() {
    os_verifier();
    root_verifier();

    log_info!("Starting os update ...");

    remount(&MountType::ReadWrite);
    mount();

    if OPTIONS.yes {
        subprocess::Exec::shell("/usr/sbin/chroot /system sh -c 'apt update && apt -y upgrade'").popen().unwrap();
    }
    else {
        subprocess::Exec::shell("/usr/sbin/chroot /system sh -c 'apt update && apt upgrade'").popen().unwrap();
    }

    log_info!("The system searches for package conflicts between system (/system) and userspace (/user)...");
    resolve_status_file_conflict();

    lock();
    remount(&MountType::ReadOnly);
    umount();

    log_info!("Finished os update ...");
}

pub fn resolve_status_file_conflict() {
    fs::copy(DPKG_USER_STATUS, format!("{}.bak", DPKG_USER_STATUS)).unwrap();

    let user_packages: HashSet<String> = {
        let file = File::open(DPKG_USER_STATUS).unwrap();
        let reader = BufReader::new(file);
        reader.lines()
            .map_while(Result::ok)
            .filter(|line| line.starts_with("Package: "))
            .map(|line| line.split_whitespace().nth(1).unwrap().to_string())
            .collect()
    };

    let mut package_entry = String::new();
    let mut infile = BufReader::new(File::open(DPKG_USER_STATUS).unwrap());
    let mut outfile = File::create(DPKG_USER_STATUS_TMP).unwrap();

    for line in infile.by_ref().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            if let Some(package_name) = package_entry.lines().find_map(|line| {
                if line.starts_with("Package: ") {
                    line.split_whitespace().nth(1).map(String::from)
                } else {
                    None
                }
            }) {
                if user_packages.contains(&package_name) {
                    let system_version = get_package_version(&package_name, DPKG_SYSTEM_STATUS);
                    let user_version = get_package_version(&package_name, DPKG_USER_STATUS);

                    if let (Some(system_version), Some(user_version)) = (system_version, user_version) {
                        if system_version != user_version {
                            log_info!(format!("Update {} from version {} to {}", package_name, user_version, system_version));
                            let updated_entry = update_version_in_entry(&package_entry, &system_version);
                            writeln!(outfile, "{}\n", updated_entry).unwrap();

                        } else {
                            writeln!(outfile, "{}\n", package_entry).unwrap();
                        }
                    } else {
                        writeln!(outfile, "{}\n", package_entry).unwrap();
                    }
                } else {
                    writeln!(outfile, "{}\n", package_entry).unwrap();
                }
            }
            package_entry.clear();
        } else {
            package_entry.push_str(&line);
            package_entry.push('\n');
        }
    }

    if !package_entry.is_empty() {
        writeln!(outfile, "{}\n", package_entry).unwrap();
    }

    fs::rename(DPKG_USER_STATUS_TMP, DPKG_USER_STATUS).unwrap();

    println!("Package status update completed");

}