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
    {CYAN}{BOLD}update:{C_RESET} Updates the system partition
    {CYAN}{BOLD}lock:{C_RESET} Lock apt package update for userspace
    {CYAN}{BOLD}unlock:{C_RESET} Unlock apt package update for userspace
    {CYAN}{BOLD}remount:{C_RESET} Remount your system partition and make it writeable
", *VERSION);
    std::process::exit(0);
}

/*
{BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-u, --use <server>{C_RESET}      Select your target server for tunneling your traffic
            {CYAN}{BOLD}-l, --local-host <host>{C_RESET} The address to expose                 {GREEN}{BOLD}[default: localhost]{C_RESET}
            {CYAN}{BOLD}-p, --port <port>{C_RESET}       The port to expose                    {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-s, --secret <secret>{C_RESET}   Secret for authentication             {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-a, --auth{C_RESET}              Use Strawberry ID for Authentication  {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-cp, --control-port{C_RESET}     Control port for remote proxy server  {GREEN}{BOLD}[default: 7835]{C_RESET}
 */