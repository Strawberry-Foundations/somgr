use subprocess::NullFile;

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
            .stderr(NullFile)
            .join()
            .unwrap_or_else(|_| {
                log_panic!("Failed to execute command");
            })
    }
}

#[macro_export]
macro_rules! subprocess {
    ($cmd:expr) => {
        subprocess($cmd)
    };
}