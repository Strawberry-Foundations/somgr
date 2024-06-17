use crate::args::{ARGS, Command, OPTIONS};

pub mod args;
pub mod commands;
pub mod statics;
pub mod util;
pub mod utilities;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    match ARGS.command {
        Command::Shell => commands::shell::shell(),
        Command::Mount => commands::mount::mount(),
        Command::Umount => commands::mount::umount(),
        Command::Remount => commands::mount::remount(&OPTIONS.mount_type),
        Command::Update => commands::update::update(),
        Command::Lock => commands::lock::lock(),
        Command::Unlock => commands::unlock::unlock(),
        Command::About => commands::about::about(),
        Command::Reboot => {
            if OPTIONS.fw {
                commands::reboot::reboot_fw()
            }
            else {
                commands::reboot::reboot()
            }
        }
        Command::Login => commands::login::login().await?,
        Command::Backup => commands::backup::main().await,
        Command::None => commands::help::help(),
    }

    Ok(())
}
