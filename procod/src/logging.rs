use std::io;
use std::io::{Write, BufWriter};
use std::error::Error;
use std::fs::{File, OpenOptions};
use log;
use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata};

pub enum FileLoggerError<'file> {
    AlreadySet,
    FileError(&'file str, String),
}

pub struct FileLogger {
    log_file: File
}

// FileLogger Methods
impl FileLogger {
    pub fn init(file_name: &str) -> Result<(), FileLoggerError> {
        let result = match open_file(file_name) {
            Ok(file) => configure_logger(file),
            Err(e) => Err(FileLoggerError::FileError(file_name, e.description().to_owned()))
        };

        result
    }

    fn get_log_file(&self) -> &File {
        &self.log_file
    }

    fn write_to_file(&self, mesg: &str) {
        let log_file = self.get_log_file();

        let mut buf_writer = BufWriter::with_capacity(1024, log_file);

        buf_writer.write(mesg.as_bytes()).unwrap();
        buf_writer.flush().unwrap();
    }
}

// Trait Log implementation
impl log::Log for FileLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mesg = format!("{} -- {}\n", record.level(), record.args());

            self.write_to_file(&mesg)
        }
    }
}

fn configure_logger<'a>(file: File) -> Result<(), FileLoggerError<'a>> {
    let result = match log::set_logger(move |max_logging_level| {
        max_logging_level.set(LogLevelFilter::Info);
        Box::new(FileLogger { log_file: file })
    }) {
        Ok(_) => Ok(()),
        Err(_) => Err(FileLoggerError::AlreadySet)
    };

    result
}

fn open_file(file_name: &str) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
}