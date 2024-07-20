use stblib::colors::{C_RESET, BOLD, UNDERLINE, WHITE, GREEN, LIGHT_GREEN, YELLOW, BLUE, RED, RESET};
use crate::statics::VERSION;

pub fn about() {
    println!("\
* ----------- {BLUE}{BOLD}somgr{C_RESET}{BOLD}{WHITE} ------------ *
|            v{}              |
|  somgr is the management tool  |
|   for StrawberryOS to update,  |
|    configure or manage your    |
|        operating system        |
* ------------------------------ *

* ------------------------------ *
|    {GREEN}Copyright{RESET} {BLUE}(C){RESET} 2022 - 2024   |
|     Strawberry Foundations     |
* ------------------------------ *

* ----------- {BOLD}{YELLOW}WARNING{C_RESET}{BOLD} ---------- *
|    This program comes with     |
|     absolutely {RED}{UNDERLINE}NO{C_RESET}{WHITE}{BOLD} warranty     |
|                                |
| {LIGHT_GREEN}This is free software, and you{C_RESET}{WHITE}{BOLD} |
| {LIGHT_GREEN}are welcome to redistribute it{C_RESET}{WHITE}{BOLD} |
* ------------------------------ *

* ------------- {BOLD}{GREEN}License{C_RESET}{WHITE}{BOLD} ------------- *
|               GPL-3.0               |
|         Open Source License         |
|                                     |
| https://opensource.org/license/gpl  |
* ----------------------------------- *{C_RESET}
", *VERSION);
    std::process::exit(0);
}