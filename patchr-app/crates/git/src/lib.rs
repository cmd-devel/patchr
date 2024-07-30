use std::fmt::{Debug, Display};

#[cfg(test)]
mod test;

pub mod patch_sender;
pub mod repo;
pub mod series;
pub mod util;

#[derive(Clone)]
pub struct GitError {
    code: GitErrorCode,
    message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum GitErrorCode {
    StringFormatError,
    InvalidPath,
    FailedToOpenRepo,
    CommandExecutionFailed,
    FailedToCreateSeries,
    SendSeriesFailed,
    SeriesAlreadyExists,
    UnknownSeries,
    RepoOpFailed,
}

impl GitError {
    pub fn new(code: GitErrorCode, message: String) -> Self {
        Self { code, message }
    }

    pub fn repo_op_failed(msg: &str) -> GitError {
        GitError::new(GitErrorCode::RepoOpFailed, String::from(msg))
    }
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Debug for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("GitError - {:?}: {}", self.code, self.message))
    }
}
