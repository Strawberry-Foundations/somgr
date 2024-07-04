use stblib::colors::{C_RESET, BOLD, UNDERLINE, CYAN};
use crate::statics::VERSION;

pub fn about() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}StrawberryOS Manager (somgr) v{}{C_RESET}\n\
{BOLD}somgr is the management tool for StrawberryOS to update, configure or manage your operating system{C_RESET}

", *VERSION);
    std::process::exit(0);
}