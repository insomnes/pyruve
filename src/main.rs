use std::env;
mod command;
mod integration;
mod venv;
use anyhow::Result;

// Temporary stored in /home/mdengin/.local/bin/pyruve
fn main() -> Result<()> {
    if env::args().len() > 1 {
        return integration::run();
    }
    let cwd = std::env::current_dir()?;
    let command = command::get_command(&cwd)?;
    print!("{}", command);

    return Ok(());
}
