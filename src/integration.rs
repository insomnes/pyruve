use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use crate::command::{self, Action};
use crate::{invalid_input, AppResult};

const DEACTIVATE_EXIT_CODE: u8 = 10;
const NOOP_EXIT_CODE: u8 = 11;

const HELP: &str = "pyruve automatically activates Python virtual environments.

Usage:
  pyruve shell <bash|fish|zsh>
  pyruve hook <bash|fish|zsh>
  pyruve --help
  pyruve --version

The 'shell' command prints shell initialization code.
The 'hook' command is an internal interface used by that code.
";

#[derive(Clone, Copy)]
enum Shell {
    Bash,
    Fish,
    Zsh,
}

pub fn run(args: &[OsString]) -> AppResult<ExitCode> {
    // pyruve 0.1 shell hooks called the binary without arguments. Keep an
    // upgraded, already-running shell inert until its configuration reloads.
    let Some(command) = args.first() else {
        return Ok(ExitCode::SUCCESS);
    };

    match command.to_str() {
        Some("shell") => shell(args),
        Some("hook") => hook(args),
        Some("--help" | "-h") => {
            print!("{HELP}");
            Ok(ExitCode::SUCCESS)
        }
        Some("--version" | "-V") => {
            println!("pyruve {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        Some(command) => Err(invalid_input(format!("unknown command: {command}"))),
        None => Err(invalid_input("command is not valid UTF-8")),
    }
}

fn shell(args: &[OsString]) -> AppResult<ExitCode> {
    let shell = parse_shell_argument(args)?;
    let shell_text = match shell {
        Shell::Bash => include_str!("shell/pyruve.bash"),
        Shell::Fish => include_str!("shell/pyruve.fish"),
        Shell::Zsh => include_str!("shell/pyruve.zsh"),
    };

    print!("{shell_text}");
    Ok(ExitCode::SUCCESS)
}

fn hook(args: &[OsString]) -> AppResult<ExitCode> {
    let shell = parse_shell_argument(args)?;
    let cwd = std::env::current_dir()?;

    match command::get_action(&cwd)? {
        Action::Activate(venv) => {
            let activation_script = activation_script(&venv, shell);
            if !activation_script.is_file() {
                return Err(invalid_input(format!(
                    "activation script does not exist: {}",
                    activation_script.display()
                )));
            }
            write_path(&activation_script)?;
            Ok(ExitCode::SUCCESS)
        }
        Action::Deactivate => Ok(ExitCode::from(DEACTIVATE_EXIT_CODE)),
        Action::Noop => Ok(ExitCode::from(NOOP_EXIT_CODE)),
    }
}

fn parse_shell_argument(args: &[OsString]) -> AppResult<Shell> {
    if args.len() != 2 {
        return Err(invalid_input(format!(
            "expected one shell argument, got {}",
            args.len().saturating_sub(1)
        )));
    }

    match args[1].to_str() {
        Some("bash") => Ok(Shell::Bash),
        Some("fish") => Ok(Shell::Fish),
        Some("zsh") => Ok(Shell::Zsh),
        Some(shell) => Err(invalid_input(format!("unsupported shell: {shell}"))),
        None => Err(invalid_input("shell name is not valid UTF-8")),
    }
}

fn activation_script(venv: &Path, shell: Shell) -> PathBuf {
    match shell {
        Shell::Bash | Shell::Zsh => venv.join("bin/activate"),
        Shell::Fish => venv.join("bin/activate.fish"),
    }
}

fn write_path(path: &Path) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    #[cfg(unix)]
    {
        let path = path.as_os_str().as_bytes();
        if path.contains(&b'\n') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "activation paths containing newlines are not supported",
            ));
        }
        stdout.write_all(path)?;
    }

    #[cfg(not(unix))]
    write!(stdout, "{}", path.display())?;

    stdout.write_all(b"\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_shell_specific_activation_script() {
        let venv = Path::new("/tmp/example with ' quote/.venv");

        assert_eq!(
            activation_script(venv, Shell::Bash),
            venv.join("bin/activate")
        );
        assert_eq!(
            activation_script(venv, Shell::Fish),
            venv.join("bin/activate.fish")
        );
    }

    #[test]
    fn rejects_extra_shell_arguments() {
        let args = vec!["shell".into(), "bash".into(), "extra".into()];
        assert!(parse_shell_argument(&args).is_err());
    }
}
