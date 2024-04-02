use std::env;
use lazy_static::lazy_static;
use stblib::colors::{BOLD, C_RESET, RED, RESET};

lazy_static!(
    pub static ref ARGS: Args = Args::collect();
    pub static ref OPTIONS: Options = Args::collect().collect_options();
);

pub enum Command {
    Shell,
    About,
    Remount,
    None
}

#[derive(Default)]
pub struct ServerOptions {
    pub min_port: u16,
    pub max_port: u16,
    pub secret: Option<String>,
    pub require_id: bool,
    pub control_port: u16,
}

#[derive(Default)]
pub struct ClientOptions {
    pub host: String,
    pub port: u16,
    pub server: String,
    pub secret: Option<String>,
    pub auth: bool,
    pub control_port: u16,
}


#[derive(Default)]
pub struct Options {
    pub server_options: ServerOptions,
    pub client_options: ClientOptions,
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

        args.args = parser.clone();
        args.command_str = parser.clone().first().unwrap().to_string();

        match args.command_str.as_str() {
            "shell" => args.command = Command::Shell,
            "remount" => args.command = Command::Remount,
            "about" => args.command = Command::About,
            _ => args.command = Command::None,
        }

        args
    }

    pub fn collect_options(&mut self) -> Options {
        let mut options = Options {
            server_options: ServerOptions {
                min_port: 1024,
                max_port: 65535,
                secret: Some(String::new()),
                require_id: false,
                control_port: 7835,
            },
            client_options: ClientOptions {
                host: String::from("localhost"),
                port: 8080,
                server: String::from("strawberryfoundations.xyz"),
                secret: Some(String::new()),
                auth: false,
                control_port: 7835,
            }
        };


        let mut iter = self.args.clone().into_iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-p" | "--port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(port) = val.parse::<u16>() {
                            options.client_options.port = port;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing port{C_RESET}");
                    }
                },

                "-u" | "--use" => {
                    if let Some(val) = iter.next() {
                        if let Ok(server) = val.parse::<String>() {
                            options.client_options.server = server;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid server{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing server{C_RESET}");
                    }
                },

                "-l" | "--local-host" => {
                    if let Some(val) = iter.next() {
                        if let Ok(host) = val.parse::<String>() {
                            options.client_options.host = host;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid local host{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing local host{C_RESET}");
                    }
                },

                "--min-port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(min_port) = val.parse::<u16>() {
                            options.server_options.min_port = min_port;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid minimal port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing minimal port{C_RESET}");
                    }
                },

                "--max-port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(max_port) = val.parse::<u16>() {
                            options.server_options.max_port = max_port;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid maximal port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing maximal port{C_RESET}");
                    }
                },

                "-a" | "--auth" => options.client_options.auth = true,
                "-id" | "--require-id" => options.server_options.require_id = true,

                _ => {
                    let port = arg.parse::<u16>().unwrap_or_else(|_| {
                        eprintln!("{RED}{BOLD} ! {RESET} Invalid port{C_RESET}");
                        std::process::exit(1);
                    });

                    options.client_options.port = port
                }
            }
        }

        options
    }
}