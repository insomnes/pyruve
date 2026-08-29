use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::process::ExitCode;

mod command;
mod integration;
mod venv;

type AppResult<T> = Result<T, Box<dyn Error>>;

fn invalid_input(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::new(io::ErrorKind::InvalidInput, message.into()))
}

fn main() -> ExitCode {
    let args: Vec<OsString> = env::args_os().skip(1).collect();

    match integration::run(&args) {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("pyruve: {error}");
            ExitCode::FAILURE
        }
    }
}
