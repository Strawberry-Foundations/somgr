use crate::args::ARGS;
use crate::commands;

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
        _ => commands::help::help()
    }
}