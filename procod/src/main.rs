#[macro_use]
extern crate log;

mod logging;

use logging::{FileLogger, FileLoggerError};

fn main() {
    match FileLogger::init("/Users/Steed/test.log") {
        Ok(_) => {
            println!("Logger initialized");
            info!("This is a fucking test.");
            info!("This is another fucking test.");
        },
        Err(e) => match e {
            FileLoggerError::AlreadySet => println!("Alread set the logger"),
            FileLoggerError::FileError(fname, msg) => println!("{:?}, {:?}", fname, msg)
        }
    }
}
