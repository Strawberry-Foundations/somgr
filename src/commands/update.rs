use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use eyre::eyre;

use crate::commands::lock::lock;
use crate::commands::mount::{mount, umount, remount, MountType};
use crate::util::verification::os_verifier;
use crate::core::dpkg::status::{get_package_version, get_package_entry};
use crate::core::args::OPTIONS;
use crate::core::fs::drop_fs_cache;
use crate::statics::{DPKG_SYSTEM_STATUS, DPKG_USER_STATUS, DPKG_USER_STATUS_TMP};
use crate::{log_info, log_ok, log_warn};
use crate::core::dpkg::serde::DpkgStatus;

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

    log_info!("The system searches for package conflicts between system (/system) and userspace (/user) ...");

    if resolve_status_file_conflict().is_err() {
        log_warn!("Cancelling update. Please install any package in userspace before updating your system");
        std::process::exit(0)
    };

    drop_fs_cache();

    lock();

    log_info!("Syncing file systems ...");
    subprocess::Exec::shell("sync").popen().unwrap();

    umount();
    remount(&MountType::ReadOnly);

    log_ok!("Finished os update ...");
}

pub fn resolve_status_file_conflict() -> eyre::Result<()> {
    // Check if dpkg userspace file exists
    if fs::metadata(DPKG_USER_STATUS).is_err() {
        log_warn!("Package status file does not exist. Aborting ...");
        return Err(eyre!("Package status file does not exist"));
    }

    // Backup original userspace status file
    fs::copy(DPKG_USER_STATUS, format!("{}.bak", DPKG_USER_STATUS)).unwrap();

    let mut user_packages = DpkgStatus::from_status_file("/var/lib/dpkg/status");
    let system_packages = DpkgStatus::from_status_file(DPKG_SYSTEM_STATUS);

    let mut user_statusfile_tmp = File::create(DPKG_USER_STATUS_TMP).unwrap();

    for package in user_packages.clone().packages {
        let package_name = package.package.clone();
        let system_package = system_packages.search_package(package_name.unwrap().as_str());

        if let Some(sys_package) = system_package {
            if let (Some(system_version), Some(user_version)) = (&sys_package.version, &package.version) {
                if system_version != user_version {
                    let package_name = package.package.clone();
                    log_info!(format!(
                        "Update {} from version {user_version} to {system_version}",
                        &package_name.unwrap().as_str()
                    ));

                    let package_name = package.package.clone();
                    let new_package = system_packages.search_package(package_name.clone().unwrap().as_str()).unwrap();

                    user_packages.update_status(&package.str, &new_package.str);
                    user_packages.write_status_file("/var/lib/dpkg/status")?;
                }
            }
        }
        else if OPTIONS.verbose {
            log_warn!(format!("Skipping package {}: Not system-wide installed", package.package.as_ref().unwrap()));
        }
    }

    // Get all installed packages in the user status file
    let _user_packages: HashSet<String> = {
        let file = File::open(DPKG_USER_STATUS).unwrap();
        let reader = BufReader::new(file);
        reader.lines()
            .map_while(Result::ok)
            .filter(|line| line.starts_with("Package: "))
            .map(|line| line.split_whitespace().nth(1).unwrap().to_string())
            .collect()
    };

    // Get all installed packages in the system status file
    let _system_packages: HashSet<String> = {
        let file = File::open(DPKG_SYSTEM_STATUS).unwrap();
        let reader = BufReader::new(file);
        reader.lines()
            .map_while(Result::ok)
            .filter(|line| line.starts_with("Package: "))
            .map(|line| line.split_whitespace().nth(1).unwrap().to_string())
            .collect()
    };

    log_info!("Checking for new packages ...");

    // Add missing packages from the system status file to the user status file
    for package_name in _system_packages.difference(&_user_packages) {
        let package_entry = get_package_entry(package_name, DPKG_SYSTEM_STATUS);
        let version = get_package_version(package_name, DPKG_SYSTEM_STATUS);

        if let (Some(entry), Some(version)) = (package_entry, version) {
            log_info!(format!("Add {package_name} version {version}"));
            writeln!(user_statusfile_tmp, "{}\n", entry).unwrap();
        }
    }

    log_ok!("Package status update completed");
    Ok(())
}