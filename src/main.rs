use crate::args::{ARGS, Command};

pub mod args;
pub mod commands;
pub mod statics;
pub mod util;

fn main() -> eyre::Result<()> {
    match ARGS.command {
        Command::Shell => commands::shell::shell(),
        Command::Remount => commands::remount::remount(),
        Command::Update => commands::update::update(),
        Command::About => commands::about::about(),
        Command::None => commands::help::help(),
    }

    Ok(())
}
