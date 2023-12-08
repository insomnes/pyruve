use std::env;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

const VENV_DIRS_ENV_VAR_NAME: &str = "PYRUVE_VENV_DIRS";
const DEFAULT_VENV_DIRS: &str = "venv,.venv";

const DELIMITERS_ENV_VAR_NAME: &str = "PYRUVE_DELIMITERS";
const DEFAULT_DELIMITERS: &str = "-,_";

const COMBINE_DIRS_ENV_VAR_NAME: &str = "PYRUVE_COMBINE_DIRS";
const DEFAULT_COMBINE_DIRS: &str = "true";

fn get_venv_dirs() -> Result<Vec<String>> {
    let dirs: Vec<String> = env::var(VENV_DIRS_ENV_VAR_NAME)
        .unwrap_or(DEFAULT_VENV_DIRS.to_string())
        .split(",")
        .map(|s| s.to_owned())
        .collect();
    if dirs.is_empty() {
        return Err(anyhow!("No venv dirs found in {}", VENV_DIRS_ENV_VAR_NAME));
    }

    if dirs.contains(&"".to_string()) {
        return Err(anyhow!(
            "Empty venv dir found in {}: {:?}",
            VENV_DIRS_ENV_VAR_NAME,
            dirs
        ));
    }
    return Ok(dirs);
}

fn get_delimiters() -> Result<Vec<String>> {
    let delimiters: Vec<String> = env::var(DELIMITERS_ENV_VAR_NAME)
        .unwrap_or(DEFAULT_DELIMITERS.to_string())
        .split(",")
        .map(|s| s.to_owned())
        .collect();
    if delimiters.is_empty() {
        return Err(anyhow!(
            "No delimiters found in {}",
            DELIMITERS_ENV_VAR_NAME
        ));
    }
    if delimiters.contains(&"".to_string()) {
        return Err(anyhow!(
            "Empty delimiter found in {}: {:?}",
            DELIMITERS_ENV_VAR_NAME,
            delimiters
        ));
    }
    return Ok(delimiters);
}

pub struct PossibleVenvChecker {
    base_venv_dir_names: Vec<String>,
    combine_dirs: bool,
    pre_combined_dirs: Vec<String>,
}

impl PossibleVenvChecker {
    pub fn build() -> Result<Self> {
        let base_venv_dir_names = get_venv_dirs()?;
        let delimiters = get_delimiters()?;

        let raw_combine_dirs = env::var(COMBINE_DIRS_ENV_VAR_NAME)
            .unwrap_or(DEFAULT_COMBINE_DIRS.to_string())
            .to_lowercase();

        let combine_dirs = match raw_combine_dirs.as_str() {
            "true" | "t" | "1" | "on" => true,
            _ => false,
        };

        let mut pre_combined_dirs: Vec<String> = Vec::new();
        if combine_dirs {
            for venv_dir in &base_venv_dir_names {
                for delimiter in &delimiters {
                    pre_combined_dirs.push(format!("{}{}", delimiter, venv_dir));
                }
            }
        }

        return Ok(PossibleVenvChecker {
            base_venv_dir_names,
            combine_dirs,
            pre_combined_dirs,
        });
    }

    pub fn search_venv_recursively(self, dir: &mut PathBuf) -> Option<PathBuf> {
        loop {
            match self.find_venv(dir) {
                Some(venv) => return Some(venv),
                None => match dir.parent() {
                    Some(parent_dir) => *dir = parent_dir.to_path_buf(),
                    None => return None,
                },
            }
        }
    }

    pub fn find_venv(&self, dir: &PathBuf) -> Option<PathBuf> {
        for venv_dir_name in self.base_venv_dir_names.iter() {
            let possibly_venv = dir.join(venv_dir_name).join("bin/activate");
            if possibly_venv.exists() {
                return Some(possibly_venv);
            }
        }
        if self.combine_dirs {
            if let Some(name) = dir.file_name() {
                let dir_name = name.to_str().unwrap();
                for venv_dir_suffix in self.pre_combined_dirs.iter() {
                    let venv_dir_name = format!("{}{}", dir_name, venv_dir_suffix);
                    let possibly_venv = dir.join(venv_dir_name).join("bin/activate");
                    if possibly_venv.exists() {
                        return Some(possibly_venv);
                    }
                }
            }
        };
        return None;
    }
}

pub fn get_active_virtual_env() -> Option<PathBuf> {
    if let Ok(var_val) = env::var("VIRTUAL_ENV") {
        let venv = PathBuf::from(var_val);
        if venv.exists() {
            return Some(venv);
        }
    }
    return None;
}
