use subprocess::{NullFile, Popen};

use crate::args::OPTIONS;
use crate::log_panic;

pub fn subprocess(cmd: impl ToString) -> subprocess::ExitStatus {
    let command = cmd.to_string();

    if OPTIONS.verbose {
        subprocess::Exec::shell(command)
            .join()
            .unwrap_or_else(|err| {
                log_panic!(format!("Failed to execute command ({err})"));
            })
    }
    else {
        subprocess::Exec::shell(command)
            .stdout(NullFile)
            .join()
            .unwrap_or_else(|_| {
                log_panic!("Failed to execute command");
            })
    }
}

pub fn popen(cmd: impl ToString) -> Popen {
    let command = cmd.to_string();

    subprocess::Exec::shell(command).popen().unwrap()
}

#[macro_export]
macro_rules! subprocess {
    ($cmd:expr) => {
        subprocess($cmd)
    };
}