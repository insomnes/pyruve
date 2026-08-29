use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::{invalid_input, AppResult};

const VENV_DIRS_ENV_VAR_NAME: &str = "PYRUVE_VENV_DIRS";
const DEFAULT_VENV_DIRS: &str = "venv,.venv";

const DELIMITERS_ENV_VAR_NAME: &str = "PYRUVE_DELIMITERS";
const DEFAULT_DELIMITERS: &str = "-,_";

const COMBINE_DIRS_ENV_VAR_NAME: &str = "PYRUVE_COMBINE_DIRS";
const DEFAULT_COMBINE_DIRS: &str = "true";

fn get_list(env_var_name: &str, default: &str, item_name: &str) -> AppResult<Vec<String>> {
    let values: Vec<String> = env::var(env_var_name)
        .unwrap_or_else(|_| default.to_owned())
        .split(',')
        .map(str::to_owned)
        .collect();

    if values.iter().any(String::is_empty) {
        return Err(invalid_input(format!(
            "empty {item_name} found in {env_var_name}: {values:?}"
        )));
    }

    Ok(values)
}

pub struct PossibleVenvChecker {
    base_venv_dir_names: Vec<String>,
    combined_dir_suffixes: Vec<String>,
}

impl PossibleVenvChecker {
    pub fn build() -> AppResult<Self> {
        let base_venv_dir_names = get_list(VENV_DIRS_ENV_VAR_NAME, DEFAULT_VENV_DIRS, "venv dir")?;
        let delimiters = get_list(DELIMITERS_ENV_VAR_NAME, DEFAULT_DELIMITERS, "delimiter")?;
        let combine_dirs = matches!(
            env::var(COMBINE_DIRS_ENV_VAR_NAME)
                .unwrap_or_else(|_| DEFAULT_COMBINE_DIRS.to_owned())
                .to_lowercase()
                .as_str(),
            "true" | "t" | "1" | "on"
        );

        Ok(Self::new(base_venv_dir_names, delimiters, combine_dirs))
    }

    pub(crate) fn new(
        base_venv_dir_names: Vec<String>,
        delimiters: Vec<String>,
        combine_dirs: bool,
    ) -> Self {
        let combined_dir_suffixes = if combine_dirs {
            base_venv_dir_names
                .iter()
                .flat_map(|venv_dir| {
                    delimiters
                        .iter()
                        .map(move |delimiter| format!("{delimiter}{venv_dir}"))
                })
                .collect()
        } else {
            Vec::new()
        };

        Self {
            base_venv_dir_names,
            combined_dir_suffixes,
        }
    }

    pub fn search_venv_recursively(&self, dir: &Path) -> Option<PathBuf> {
        dir.ancestors()
            .find_map(|candidate| self.find_venv(candidate))
    }

    fn find_venv(&self, dir: &Path) -> Option<PathBuf> {
        for venv_dir_name in &self.base_venv_dir_names {
            let possible_venv = dir.join(venv_dir_name);
            if is_venv(&possible_venv) {
                return Some(possible_venv);
            }
        }

        let dir_name = dir.file_name()?;
        for suffix in &self.combined_dir_suffixes {
            let mut venv_dir_name = OsString::from(dir_name);
            venv_dir_name.push(suffix);
            let possible_venv = dir.join(venv_dir_name);
            if is_venv(&possible_venv) {
                return Some(possible_venv);
            }
        }

        None
    }
}

fn is_venv(path: &Path) -> bool {
    path.join("bin/activate").is_file()
}

pub fn get_active_virtual_env() -> Option<PathBuf> {
    let venv = PathBuf::from(env::var_os("VIRTUAL_ENV")?);
    venv.is_dir().then_some(venv)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::os::unix::ffi::OsStringExt;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIR: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn searches_non_utf8_directories_without_panicking() {
        let sequence = NEXT_TEST_DIR.fetch_add(1, Ordering::Relaxed);
        let root = env::temp_dir().join(format!(
            "pyruve-venv-test-{}-{sequence}",
            std::process::id()
        ));
        let project = root.join(OsString::from_vec(b"project-\xff".to_vec()));
        let venv = project.join(OsString::from_vec(b"project-\xff-venv".to_vec()));
        fs::create_dir_all(venv.join("bin")).unwrap();
        File::create(venv.join("bin/activate")).unwrap();

        let checker = PossibleVenvChecker::new(vec!["venv".to_owned()], vec!["-".to_owned()], true);
        assert_eq!(checker.search_venv_recursively(&project), Some(venv));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ignores_activation_directories() {
        let sequence = NEXT_TEST_DIR.fetch_add(1, Ordering::Relaxed);
        let root = env::temp_dir().join(format!(
            "pyruve-venv-test-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(root.join(".venv/bin/activate")).unwrap();

        let checker = PossibleVenvChecker::new(vec![".venv".to_owned()], Vec::new(), false);
        assert_eq!(checker.search_venv_recursively(&root), None);

        fs::remove_dir_all(root).unwrap();
    }
}
