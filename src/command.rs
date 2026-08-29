use std::fs;
use std::path::{Path, PathBuf};

use crate::venv::{self, PossibleVenvChecker};
use crate::AppResult;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Activate(PathBuf),
    Deactivate,
    Noop,
}

pub fn get_action(cwd: &Path) -> AppResult<Action> {
    let checker = PossibleVenvChecker::build()?;
    let active_venv = venv::get_active_virtual_env();

    Ok(select_action(cwd, active_venv.as_deref(), &checker))
}

fn select_action(cwd: &Path, active_venv: Option<&Path>, checker: &PossibleVenvChecker) -> Action {
    let candidate = checker.search_venv_recursively(cwd);

    match (candidate, active_venv) {
        (Some(candidate), Some(active)) if paths_match(&candidate, active) => Action::Noop,
        (Some(candidate), _) => Action::Activate(candidate),
        (None, Some(active)) if is_inside_venv_project(cwd, active) => Action::Noop,
        (None, Some(_)) => Action::Deactivate,
        (None, None) => Action::Noop,
    }
}

fn paths_match(left: &Path, right: &Path) -> bool {
    left == right
        || matches!(
            (fs::canonicalize(left), fs::canonicalize(right)),
            (Ok(left), Ok(right)) if left == right
        )
}

fn is_inside_venv_project(cwd: &Path, active_venv: &Path) -> bool {
    match active_venv.parent() {
        Some(project_dir) => cwd.starts_with(project_dir),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIR: AtomicU64 = AtomicU64::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let sequence = NEXT_TEST_DIR.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "pyruve-command-test-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn checker() -> PossibleVenvChecker {
        PossibleVenvChecker::new(
            vec!["venv".to_owned(), ".venv".to_owned()],
            vec!["-".to_owned(), "_".to_owned()],
            true,
        )
    }

    fn create_venv(root: &Path) -> PathBuf {
        let bin = root.join("bin");
        fs::create_dir_all(&bin).unwrap();
        File::create(bin.join("activate")).unwrap();
        File::create(bin.join("activate.fish")).unwrap();
        root.to_path_buf()
    }

    #[test]
    fn activates_nearest_nested_venv() {
        let root = TestDir::new();
        let outer = create_venv(&root.path().join(".venv"));
        let nested_project = root.path().join("nested");
        let nested = create_venv(&nested_project.join(".venv"));
        let cwd = nested_project.join("src");
        fs::create_dir_all(&cwd).unwrap();

        assert_eq!(
            select_action(&cwd, Some(&outer), &checker()),
            Action::Activate(nested)
        );
    }

    #[test]
    fn keeps_matching_active_venv() {
        let root = TestDir::new();
        let active = create_venv(&root.path().join(".venv"));
        let cwd = root.path().join("src");
        fs::create_dir_all(&cwd).unwrap();

        assert_eq!(select_action(&cwd, Some(&active), &checker()), Action::Noop);
    }

    #[test]
    fn deactivates_outside_active_project() {
        let root = TestDir::new();
        let project = root.path().join("project");
        let active = create_venv(&project.join("custom-env"));
        let elsewhere = root.path().join("elsewhere");
        fs::create_dir_all(&elsewhere).unwrap();

        assert_eq!(
            select_action(&elsewhere, Some(&active), &checker()),
            Action::Deactivate
        );
    }

    #[test]
    fn keeps_custom_venv_inside_its_project() {
        let root = TestDir::new();
        let project = root.path().join("project");
        let active = create_venv(&project.join("custom-env"));
        let cwd = project.join("src");
        fs::create_dir_all(&cwd).unwrap();

        assert_eq!(select_action(&cwd, Some(&active), &checker()), Action::Noop);
    }
}
