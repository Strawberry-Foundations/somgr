use crate::args::{ARGS, Command, OPTIONS};

pub mod args;
pub mod commands;
pub mod statics;
pub mod util;

fn main() -> eyre::Result<()> {
    match ARGS.command {
        Command::Shell => commands::shell::shell(),
        Command::Mount => commands::mount::mount(),
        Command::Umount => commands::mount::umount(),
        Command::Remount => commands::mount::remount(&OPTIONS.mount_type),
        Command::Update => commands::update::update(),
        Command::Lock => commands::lock::lock(),
        Command::Unlock => commands::unlock::unlock(),
        Command::About => commands::about::about(),
        Command::None => commands::help::help(),
    }

    Ok(())
}
