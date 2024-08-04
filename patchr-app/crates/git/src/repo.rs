use std::{fs, path::Path};

use common::constants::PROJECT_VERSION;
use serde::{Deserialize, Serialize};

use crate::{series::Series, util::GitRepo, GitError, GitErrorCode};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoMetadata {
    name: String,
    path: String,
}

// This is the serializable repo struct
// It offers a way to open the underlying repo
#[derive(Serialize, Deserialize)]
pub struct RepoData {
    repo: Repo,
    meta: RepoMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct Repo {
    version: String,
    series: Vec<Series>,
}

impl RepoMetadata {
    pub fn new(name: &str, path: &str) -> Self {
        let abs_path = fs::canonicalize(path).expect("Tried to find canonicalize an invalid path");
        Self {
            name: String::from(name),
            path: abs_path.to_string_lossy().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn dirname(&self) -> String {
        Path::new(self.path.as_str())
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .to_string()
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

impl PartialEq for RepoMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

impl Eq for RepoMetadata {}

impl RepoData {
    pub fn new(meta: RepoMetadata, repo: Repo) -> Self {
        Self { repo, meta }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn repo_mut(&mut self) -> &mut Repo {
        &mut self.repo
    }

    pub fn meta(&self) -> &RepoMetadata {
        &self.meta
    }

    pub fn open_git_repo(&self) -> Option<GitRepo> {
        GitRepo::open(self.meta.path.as_str())
    }
}

impl Repo {
    pub fn new() -> Self {
        Self {
            version: String::from(PROJECT_VERSION),
            series: Vec::new(),
        }
    }

    pub fn series(&self) -> &[Series] {
        self.series.as_slice()
    }

    pub fn add_series(
        &mut self, name: &str, title: &str, short_name: Option<&str>, cv_skel: Option<&str>,
    ) -> Result<(), GitError> {
        let Some(mut series) = Series::new(name, title) else {
            return Err(GitError::new(
                GitErrorCode::FailedToCreateSeries,
                String::from("Invalid inputs"),
            ));
        };
        if self.series.iter().find(|&s| s.name() == name).is_some() {
            return Err(GitError::new(
                GitErrorCode::SeriesAlreadyExists,
                String::from("Series already exists"),
            ));
        }
        if let Some(s) = short_name {
            series.set_short_name(s)?;
        }
        if let Some(s) = cv_skel {
            series.set_cover_letter(s)?;
        }
        self.series.push(series);
        Ok(())
    }

    pub fn delete_series(&mut self, name: &str) -> Result<(), GitError> {
        let count = self.series.len();
        self.series.retain(|s| s.name() != name);
        if count == self.series.len() {
            return Err(GitError::new(
                GitErrorCode::UnknownSeries,
                format!("The series named '{}' does not exist in this repo", name),
            ));
        }
        Ok(())
    }

    pub fn get_series_by_name(&self, name: &str) -> Option<&Series> {
        self.series.iter().find(|&s| s.name() == name)
    }

    pub fn get_series_by_name_mut(&mut self, name: &str) -> Option<&mut Series> {
        self.series.iter_mut().find(|s| s.name() == name)
    }

    pub fn version(&self) -> &str {
        self.version.as_str()
    }
}
