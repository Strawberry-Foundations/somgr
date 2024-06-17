use stblib::colors::{C_RESET, GREEN, BOLD, UNDERLINE, CYAN, RESET, WHITE, RED, MAGENTA};
use crate::statics::VERSION;

pub fn help() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}StrawberryOS Manager (somgr) v{}{C_RESET}\n\
{GREEN}{BOLD}Usage:{RESET} {WHITE}somgr {CYAN}[command] {RED}[<options>]{C_RESET}\n\n\
{MAGENTA}{BOLD}Commands:{C_RESET}
    {CYAN}{BOLD}help:{C_RESET} Prints this message
    {CYAN}{BOLD}about:{C_RESET} About StrawberryOS Manager
    {CYAN}{BOLD}shell:{C_RESET} Opens a new system shell
    {CYAN}{BOLD}login:{C_RESET} Login to your Strawberry ID to enable StrawberryOS Backups
    {CYAN}{BOLD}lock:{C_RESET} Lock apt package update for userspace
    {CYAN}{BOLD}unlock:{C_RESET} Unlock apt package update for userspace
    {CYAN}{BOLD}mount:{C_RESET} Bind /dev, /sys & /proc to your system partition
    {CYAN}{BOLD}umount:{C_RESET} Unmount bindings in /system

    {CYAN}{BOLD}remount:{C_RESET} Remount your system partition in a specific mode
    {BOLD}↳ {MAGENTA}Options:{C_RESET}
        {CYAN}{BOLD}-ro, --readonly{C_RESET}   Remount in readonly (ro) mode     {GREEN}{BOLD}[default]{C_RESET}
        {CYAN}{BOLD}-rw, --readwrite{C_RESET}  Remount in readwrite (rw) mode

    {CYAN}{BOLD}update:{C_RESET} Updates the system partition
    {BOLD}↳ {MAGENTA}Options:{C_RESET}
        {CYAN}{BOLD}-y, --yes{C_RESET}  Automatically accept package updates

    {CYAN}{BOLD}reboot:{C_RESET} Reboot system
    {BOLD}↳ {MAGENTA}Options:{C_RESET}
        {CYAN}{BOLD}-fw, --firmware{C_RESET}  Reboot into firmware settings

    {CYAN}{BOLD}backup:{C_RESET} StrawberryOS Backup management
    {BOLD}↳ {MAGENTA}Subcommands:{C_RESET}
        {CYAN}{BOLD}restore{C_RESET}  Restore a previously created backup
        {CYAN}{BOLD}upload {C_RESET}  Create a new backup and upload it
        {CYAN}{BOLD}add    {C_RESET}  Add new files to the backup
        {CYAN}{BOLD}remove {C_RESET}  Remove files from backup
        {CYAN}{BOLD}list   {C_RESET}  Show backups & files
", *VERSION);
    std::process::exit(0);
}