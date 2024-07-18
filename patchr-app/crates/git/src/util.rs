use std::{fmt::Display, path::PathBuf};

use crate::GitError;

// Define wrappers so that we do not expose libgit2
// structs to the rest of the code

pub struct GitRepo {
    repo: git2::Repository,
}

pub struct Commit<'a> {
    commit: git2::Commit<'a>,
    repo: &'a GitRepo,
}

pub struct CommitId {
    oid: git2::Oid,
}

#[derive(Clone)]
pub enum CommitTag {
    ReviewedBy,
    SignedOffBy,
    Custom(String),
}

impl CommitId {
    pub fn new(hex: &str) -> Result<Self, GitError> {
        let oid = match git2::Oid::from_str(hex) {
            Ok(o) => o,
            Err(e) => {
                return Err(GitError::repo_op_failed(e.message()));
            }
        };
        Ok(Self {
            oid
        })
    }

    fn from_oid(oid: git2::Oid) -> Self {
        Self { oid }
    }
}

impl Display for CommitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.oid.to_string().as_str())
    }
}

impl Display for CommitTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReviewedBy => f.write_str("Reviewed-by")?,
            Self::SignedOffBy => f.write_str("Signed-off-by")?,
            Self::Custom(tag) => f.write_fmt(format_args!("{}", tag))?,
        }
        Ok(())
    }
}

impl From<&str> for CommitTag {
    fn from(value: &str) -> Self {
        match value {
            "rb" => CommitTag::ReviewedBy,
            "so" => CommitTag::SignedOffBy,
            _ => CommitTag::Custom(String::from(value)),
        }
    }
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
            if !func(&Commit::new(commit, self)) {
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn find_commit(&self, commit: &CommitId) -> Result<Commit, GitError> {
        match self.repo.find_commit(commit.oid) {
            Ok(c) => {
                return Ok(Commit::new(c, self))
            }
            Err(e) => {
                return Err(GitError::repo_op_failed(e.message()))
            }
        }
        
    }
}

impl<'a> Commit<'a> {
    fn new(commit: git2::Commit<'a>, repo: &'a GitRepo) -> Self {
        Commit { commit, repo }
    }

    pub fn id(&self) -> CommitId {
        CommitId::from_oid(self.commit.id())
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
