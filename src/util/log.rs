#[macro_export]
macro_rules! log_info {
    ($string:expr) => {
        println!("\x1b[1m\x1b[42m   INFO   \x1b[0m  {}\x1b[0m", $string);
    };
}