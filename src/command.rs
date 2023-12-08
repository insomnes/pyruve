use std::path::PathBuf;

use anyhow::Result;

use crate::venv;

pub fn get_command(cwd: &PathBuf) -> Result<String> {
    let active_venv = venv::get_active_virtual_env();
    match active_venv {
        Some(venv) => active_route(&venv, &cwd),
        None => inactive_route(cwd),
    }
}

fn active_route(venv: &PathBuf, cwd: &PathBuf) -> Result<String> {
    let mut command = "".to_string();

    let venv_parent = match venv.parent() {
        Some(parent) => parent,
        None => return Ok(command),
    };

    if !cwd.starts_with(venv_parent) {
        command = inactive_route(cwd)?;
        if command == "".to_string() {
            command = "deactivate".to_string();
        }
    }

    return Ok(command);
}

fn inactive_route(cwd: &PathBuf) -> Result<String> {
    let mut command = "".to_string();

    let venv_checker = venv::PossibleVenvChecker::build()?;
    if let Some(script_path) = venv_checker.search_venv_recursively(&mut cwd.clone()) {
        let script = script_path.to_string_lossy().to_string();
        command = format!("source '{}'", script);
    }
    return Ok(command);
}
