use stblib::colors::{C_RESET, BOLD, UNDERLINE, CYAN};
use crate::statics::VERSION;

pub fn about() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}StrawberryOS Manager (somgr) v{VERSION}{C_RESET}\n\
{BOLD}somgr is the management tool for StrawberryOS to configure or update your operating system. {C_RESET}

");
    std::process::exit(0);
}