#[macro_export]
macro_rules! log_info {
    ($string:expr) => {
        println!("\x1b[1m\x1b[42m   INFO   \x1b[0m  {}\x1b[0m", $string);
    };
}

#[macro_export]
macro_rules! log_ok {
    ($string:expr) => {
        println!("\x1b[1m\x1b[44m    OK    \x1b[0m  {}\x1b[0m", $string);
    };
}

#[macro_export]
macro_rules! log_fail {
    ($string:expr) => {
        println!("\x1b[1m\x1b[41m   FAIL   \x1b[0m  {}\x1b[0m", $string);
    };
}

#[macro_export]
macro_rules! log_panic {
    ($string:expr) => {
        println!("\x1b[1m\x1b[41m   FAIL   \x1b[0m  {}\x1b[0m", $string);
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($string:expr) => {
        println!("\x1b[1m\x1b[43m   WARN   \x1b[0m  {}\x1b[0m", $string);
    };
}