use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use eyre::eyre;

use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount, remount, MountType};
use crate::util::verification::os_verifier;
use crate::core::dpkg::{get_package_version, update_version_in_entry};
use crate::core::args::OPTIONS;
use crate::statics::{DPKG_SYSTEM_STATUS, DPKG_USER_STATUS, DPKG_USER_STATUS_TMP};
use crate::{log_info, log_ok, log_warn};

pub fn update() {
    os_verifier();
    karen::escalate_if_needed().unwrap();

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

    if resolve_status_file_conflict().is_err() {
        log_warn!("Cancelling update. Please install any package in userspace before updating your system");
        std::process::exit(0)
    };

    lock();

    log_info!("Syncing file systems ...");
    subprocess::Exec::shell("sync").popen().unwrap();

    umount();
    remount(&MountType::ReadOnly);

    log_ok!("Finished os update ...");
}

pub fn resolve_status_file_conflict() -> eyre::Result<()> {
    // check if dpkg userspace file exists
    if fs::metadata(DPKG_USER_STATUS).is_err() {
        log_warn!("Package status file does not exist. Aborting...");
        return Err(eyre!("Package status file does not exist"));
    }

    // backup original userspace status file
    fs::copy(DPKG_USER_STATUS, format!("{}.bak", DPKG_USER_STATUS)).unwrap();

    // get all installed packages
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
    let mut statusfile = BufReader::new(File::open(DPKG_USER_STATUS).unwrap());
    let mut statusfile_temp = File::create(DPKG_USER_STATUS_TMP).unwrap();

    for line in statusfile.by_ref().lines() {
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
                            writeln!(statusfile_temp, "{}\n", updated_entry).unwrap();

                        } else {
                            writeln!(statusfile_temp, "{}\n", package_entry).unwrap();
                        }
                    } else {
                        writeln!(statusfile_temp, "{}\n", package_entry).unwrap();
                    }
                } else {
                    writeln!(statusfile_temp, "{}\n", package_entry).unwrap();
                }
            }
            package_entry.clear();
        } else {
            package_entry.push_str(&line);
            package_entry.push('\n');
        }
    }

    if !package_entry.is_empty() {
        writeln!(statusfile_temp, "{}\n", package_entry).unwrap();
    }

    fs::rename(DPKG_USER_STATUS_TMP, DPKG_USER_STATUS).unwrap();
    
    log_info!("Package status update completed");
    Ok(())
}