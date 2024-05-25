use std::{
    env,
    fmt::{Debug, Display},
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use git::repo::{Repo, RepoData, RepoMetadata};
use homedir::get_my_home;
use log::{debug, trace};

use uuid::Uuid;

use super::{mailing_list::MailingList, root_file::{RootFile, UserConfig, ROOT_FILE_NAME}};

const USER_DATA_DIR: &str = ".patchr";

pub struct UserData {
    root_file: RootFile,
    repo: Option<RepoData>,
}

#[derive(Debug, Clone, Copy)]
pub enum UserDataErrorCode {
    RepoAlreadyExists,
    RepoDoesNotExist,
    ListDoesNotExist,
    ListAlreadyExists,
    NotAGitRepo,
    FailedToSaveRootFile,
    FailedToSaveData,
    FailedToReadData,
    FsError,
    InputError,
}

#[derive(Clone)]
pub struct UserDataError {
    code: UserDataErrorCode,
    message: Option<String>,
}

pub fn root_file_path() -> Result<PathBuf, UserDataError> {
    let mut r = root_file_dir_path()?;
    r.push(ROOT_FILE_NAME);
    Ok(r)
}

pub fn root_file_dir_path() -> Result<PathBuf, UserDataError> {
    match get_my_home() {
        Ok(opt_home) => {
            if let Some(mut p) = opt_home {
                p.push(USER_DATA_DIR);
                Ok(p)
            } else {
                Err(UserDataError::new_with_message(
                    UserDataErrorCode::FsError,
                    String::from("Failed to get home dir"),
                ))
            }
        }
        Err(e) => Err(UserDataError::new_with_message(UserDataErrorCode::FsError, e.to_string())),
    }
}

pub fn root_tmp_dir_path() -> Result<PathBuf, UserDataError> {
    let mut r = root_file_dir_path()?;
    r.push("tmp");
    if !fs::metadata(&r).is_ok_and(|m| m.is_dir()) {
        if let Err(e) = fs::create_dir_all(&r) {
            return Err(UserDataError::new_with_message(
                UserDataErrorCode::FsError,
                format!("Failed to create a temporary directory : {}", e),
            ));
        }
    }
    Ok(r)
}

pub fn new_root_tmp_child_path() -> Result<PathBuf, UserDataError> {
    let mut r = root_tmp_dir_path()?;
    r.push(Uuid::new_v4().to_string());
    Ok(r)
}

impl UserData {
    pub fn load() -> Result<Self, UserDataError> {
        let root_file_path = root_file_path()?;
        debug!("load user data from {:?}", root_file_path);
        if !root_file_path.exists() {
            // make sure the directory exists, the file will be created when saving the returned UserData struct
            fs::create_dir_all(root_file_path.parent().unwrap()).or_else(|e| {
                Err(UserDataError::new_with_message(UserDataErrorCode::FsError, e.to_string()))
            })?;
        }
        let root_file = Self::read_or_create_root_file()?;
        let repo = UserData::find_current_repo(&root_file)?;
        Ok(Self { root_file, repo })
    }

    fn read_or_create_root_file() -> Result<RootFile, UserDataError> {
        let Ok(file) = File::open(root_file_path()?) else {
            return Ok(RootFile::new());
        };
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| {
            UserDataError::new_with_message(UserDataErrorCode::FailedToReadData, e.to_string())
        })
    }

    pub fn save_root_file(&self) -> Result<(), UserDataError> {
        trace!("Save root file");
        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(root_file_path()?)
        {
            Ok(file) => {
                let mut writer = BufWriter::new(file);
                if let Err(e) = serde_json::to_writer_pretty(&mut writer, &self.root_file) {
                    Err(UserDataError::new_with_message(
                        UserDataErrorCode::FailedToSaveRootFile,
                        e.to_string(),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(UserDataError::new_with_message(
                UserDataErrorCode::FailedToSaveRootFile,
                e.to_string(),
            )),
        }
    }

    pub fn save_repo(&self) -> Result<(), UserDataError> {
        trace!("Save repo");
        let Some(repo) = self.repo.as_ref() else {
            return Ok(()); // no repo to save
        };

        let path = UserData::get_repo_data_file_path(repo.meta().name())?;
        match File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
        {
            Ok(file) => {
                let mut writer = BufWriter::new(file);
                if let Err(_e) = serde_json::to_writer_pretty(&mut writer, repo.repo()) {
                    Err(UserDataError::new_with_message(
                        UserDataErrorCode::FailedToSaveData,
                        String::from("Failed to save the repo"),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                return Err(UserDataError::new_with_message(
                    UserDataErrorCode::FailedToSaveData,
                    e.to_string(),
                ));
            }
        }
    }

    fn find_current_repo(root_file: &RootFile) -> Result<Option<RepoData>, UserDataError> {
        let current_dir = env::current_dir()
            .ok()
            .ok_or(UserDataError::new_with_message(
                UserDataErrorCode::FsError,
                String::from("Cannot get the current directory"),
            ))?;
        if let Some(meta) =
            root_file.find_repo_by_path(current_dir.to_string_lossy().to_string().as_str())
        {
            let repo_data_file_path = UserData::get_repo_data_file_path(meta.name())?;
            if !fs::metadata(&repo_data_file_path).is_ok_and(|m| m.is_file()) {
                return Ok(None); // not in a repo
            }
            match File::open(&repo_data_file_path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    match serde_json::from_reader(reader) {
                        Ok(data) => {
                            trace!("Repo found: {}", meta.path());
                            Ok(Some(RepoData::new(meta, data)))
                        }
                        Err(e) => Err(UserDataError::new_with_message(
                            UserDataErrorCode::FailedToReadData,
                            e.to_string(),
                        )),
                    }
                }
                Err(e) => Err(UserDataError::new_with_message(
                    UserDataErrorCode::FailedToReadData,
                    e.to_string(),
                )),
            }
        } else {
            trace!("Not in a repo");
            Ok(None)
        }
    }

    pub fn register_repo(&mut self, name: &str, path: &str) -> Result<(), UserDataError> {
        match self.root_file.register_repo(name, path) {
            Ok(meta) => {
                // set the current repo so that the repo file is created on exit
                self.repo = Some(RepoData::new(meta.clone(), Repo::new()));
                Ok(())
            }
            Err(e) => {
                debug!("{}", e);
                Err(e)
            }
        }
    }

    pub fn delete_repo(&mut self, name: &str) -> Result<(), UserDataError> {
        self.root_file.delete_repo(name)?;
        self.repo = None;
        Ok(())
    }

    pub fn repos(&self) -> &[RepoMetadata] {
        self.root_file.repos()
    }

    fn get_repo_data_file_path(repo_name: &str) -> Result<PathBuf, UserDataError> {
        let mut r = root_file_dir_path()?;
        r.push(repo_name);
        Ok(r)
    }

    pub fn repo(&self) -> Option<&RepoData> {
        self.repo.as_ref()
    }

    pub fn repo_mut(&mut self) -> Option<&mut RepoData> {
        self.repo.as_mut()
    }

    pub fn config(&self) -> &UserConfig {
        self.root_file.config()
    }

    pub fn config_mut(&mut self) -> &mut UserConfig {
        self.root_file.config_mut()
    }

    pub fn add_mailing_list(&mut self, name: &str, email: &str) -> Result<(), UserDataError> {
        match self.root_file.add_mailing_list(name, email) {
            Ok(()) => Ok(()),
            Err(e) => {
                debug!("{}", e);
                Err(e)
            }
        }
    }

    pub fn delete_mailing_list(&mut self, name: &str) -> Result<(), UserDataError> {
        self.root_file.delete_mailing_list(name)
    }

    pub fn find_mailing_list(&self, name : &str) -> Option<&MailingList> {
        self.root_file.find_mailing_list(name)
    }
}

impl UserDataError {
    pub fn new(code: UserDataErrorCode) -> Self {
        Self {
            code,
            message: None,
        }
    }

    pub fn new_with_message(code: UserDataErrorCode, message: String) -> Self {
        Self {
            code,
            message: Some(message),
        }
    }
}

// TODO: add add default error messages to avoid printing the code directly
// when there is no message
impl Display for UserDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = self.message.as_ref() {
            f.write_fmt(format_args!("{:?}: {}", self.code, message))
        } else {
            f.write_fmt(format_args!("{:?}", self.code))
        }
    }
}
