use std::env;
use lazy_static::lazy_static;
use crate::commands::mount::MountType;

lazy_static!(
    pub static ref ARGS: Args = Args::collect();
    pub static ref OPTIONS: Options = Args::collect().collect_options();
);

pub enum Command {
    Shell,
    About,
    Mount,
    Umount,
    Remount,
    Update,
    Lock,
    Unlock,
    None
}

#[derive(Default)]
pub struct Options {
    pub mount_type: MountType,
}

pub struct Args {
    pub args: Vec<String>,
    pub command: Command,
    pub command_str: String,
    pub options: Options,
}

impl Args {
    pub fn collect() -> Self {
        let mut args = Self {
            args: vec![],
            command: Command::None,
            command_str: String::new(),
            options: Options { ..Default::default() }
        };

        let collector: Vec<String> = env::args().collect();

        if collector.len() <= 1 {
            return args
        }

        let parser: Vec<String> = env::args().skip(1).collect();

        args.args.clone_from(&parser);
        args.command_str = parser.clone().first().unwrap().to_string();

        match args.command_str.as_str() {
            "shell" => args.command = Command::Shell,
            "mount" => args.command = Command::Mount,
            "umount" => args.command = Command::Umount,
            "remount" => args.command = Command::Remount,
            "update" => args.command = Command::Update,
            "lock" => args.command = Command::Lock,
            "unlock" => args.command = Command::Unlock,
            "about" => args.command = Command::About,
            _ => args.command = Command::None,
        }

        args
    }

    pub fn collect_options(&mut self) -> Options {
        let mut options = Options::default();


        let mut iter = self.args.clone().into_iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                /* "-p" | "--port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(port) = val.parse::<u16>() {
                            options.client_options.port = port;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing port{C_RESET}");
                    }
                }, */

                "-ro" | "--readonly" => options.mount_type = MountType::ReadOnly,
                "-rw" | "--readwrite" => options.mount_type = MountType::ReadWrite,

                _ => {
     
                }
            }
        }

        options
    }
}