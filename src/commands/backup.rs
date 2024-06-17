use stblib::colors::{BOLD, C_RESET, RED};

use crate::args::ARGS;
use crate::commands;
use crate::commands::login::Credentials;

pub async fn main() {
    let credentials = match Credentials::read() {
        Ok(creds) => creds,
        Err(..) => {
            println!("{RED}{BOLD}Please authenticate with your Strawberry ID before using somgr's Backup function{C_RESET}");
            return;
        }
    };
    
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
        _ => commands::help::help()
    }
}

pub async fn status(credentials: Credentials) {
    
}