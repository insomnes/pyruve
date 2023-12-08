use std::env;

use anyhow::{anyhow, Result};

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("No command provided"));
    }
    let command: &String = &args[1];
    match command.as_str() {
        "shell" => shell(args),
        _ => Err(anyhow!("Unknown command: {}", *command)),
    }?;

    return Ok(());
}

fn shell(args: Vec<String>) -> Result<()> {
    if args.len() < 3 {
        return Err(anyhow!("No shell provided"));
    }
    let shell: &String = &args[2];

    let fish_text = include_str!("shell/pyruve.fish");
    let zsh_text = include_str!("shell/pyruve.zsh");
    let bash_text = include_str!("shell/pyruve.bash");

    match shell.as_str() {
        "fish" => print!("{}", fish_text),
        "zsh" => print!("{}", zsh_text),
        "bash" => print!("{}", bash_text),
        _ => return Err(anyhow!("Unknown shell: {}", *shell)),
    };
    return Ok(());
}
