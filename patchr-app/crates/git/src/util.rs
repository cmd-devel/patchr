use std::path::PathBuf;

pub fn find_repo_root(path: &str) -> Option<PathBuf> {
    if let Ok(r) = git2::Repository::open(path) {
        if r.is_bare() {
            None // Not supported
        } else {
            Some(PathBuf::from(r.path().parent().unwrap()))
        }
    } else {
        None
    }
}