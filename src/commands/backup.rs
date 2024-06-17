use stblib::colors::{BOLD, C_RESET, RED};

use crate::args::ARGS;
use crate::commands;
use crate::commands::login::Credentials;

pub fn main() {
    match ARGS.subcommand.as_str() {
        "restore" => {

        }
        "upload" => {

        }
        "add" => {

        }
        "remove" => {

        }
        "list" => {

        }
        "status" => {

        }
        _ => {
            if Credentials::read().is_ok() {
                commands::help::help()
            } else {
                println!("{RED}{BOLD}Please authenticate with your Strawberry ID before using somgr's Backup function{C_RESET}")
            }
        }
    }
}