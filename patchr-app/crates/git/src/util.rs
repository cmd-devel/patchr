use std::path::PathBuf;

use crate::GitError;

// Define wrappers so that we do not expose libgit2
// structs to the rest of the code

pub struct GitRepo {
    repo: git2::Repository,
}

pub struct Commit<'a> {
    commit: git2::Commit<'a>,
}

impl GitRepo {
    pub fn open(path: &str) -> Option<Self> {
        let repo = git2::Repository::open(path).ok()?;
        Some(GitRepo { repo })
    }

    pub fn walk_from_head<F: FnMut(&Commit) -> bool>(&self, func: &mut F) -> Result<(), GitError> {
        let mut revwalk = self.repo.revwalk().ok().ok_or(
            GitError::repo_op_failed("Failed to initialize the iterator")
        )?;
        revwalk.push_head().ok().ok_or(GitError::repo_op_failed("Failed to initialize the iterator"))?;
        for roid in revwalk {
            let Ok(oid) = roid else {
                return Err(GitError::repo_op_failed("Failed retrieve the new commit id"));
            };
            let Ok(commit) = self.repo.find_commit(oid) else {
                return Err(GitError::repo_op_failed(format!("Failed to find the commit with hash {}", oid.to_string()).as_str()));
            };
            if !func(&Commit::new(commit)) {
                return Ok(());
            }
        }
        Ok(())
    }
}

impl<'a> Commit<'a> {
    fn new(commit: git2::Commit<'a>) -> Self {
        Commit { commit }
    }

    pub fn hash(&self) -> String {
        self.commit.id().to_string()
    }

    pub fn short_name(&self) -> &str {
        self.commit.summary().unwrap_or("")
    }
}

pub fn find_repo_root(path: &str) -> Option<PathBuf> {
    let mut p = PathBuf::from(path);
    loop {
        if let Ok(r) = git2::Repository::open(&p) {
            if r.is_bare() {
                return None; // Not supported
            } else {
                return Some(p);
            }
        }

        p = match p.parent() {
            Some(p) => p.to_path_buf(),
            None => return None,
        };
    }
}
