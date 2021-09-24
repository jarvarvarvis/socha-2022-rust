use flexi_logger::{Duplicate, FileSpec, Logger};

use super::error::Error;

pub fn setup_logger() -> Result<(), Error> {
    let file_spec = FileSpec::default().directory("log/");
    let logger_with_str = Logger::try_with_str("debug");
    match logger_with_str {
        Ok(logger) => {
            let start_result = logger
                .log_to_file(file_spec)
                .duplicate_to_stderr(Duplicate::All)
                .start();

            match start_result {
                Ok(_) => Ok(()),
                Err(error) => Err(Error::LoggerError(error)),
            }
        }
        Err(error) => Err(Error::LoggerError(error)),
    }
}
