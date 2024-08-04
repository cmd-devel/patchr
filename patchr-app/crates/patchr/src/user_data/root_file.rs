use std::collections::HashMap;

use common::constants::PROJECT_VERSION;
use git::{repo::RepoMetadata, util::find_repo_root};
use serde::{Deserialize, Serialize};

use super::{
    mailing_list::MailingList,
    user_data::{UserDataError, UserDataErrorCode},
};

pub const ROOT_FILE_NAME: &str = "root.json";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserConfig {
    editor: Option<String>,
    send_command: Option<String>,
    from_email: Option<String>,
    smtp_server: Option<String>,
    smtp_user: Option<String>,
    smtp_port: Option<u16>,
    smtp_encryption: Option<String>,
    cv_skel: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RootFile {
    version: String,
    user_config: UserConfig,
    lists: HashMap<String, MailingList>,
    repos: Vec<RepoMetadata>,
}

impl UserConfig {
    pub fn new() -> Self {
        Self {
            editor: None,
            send_command: None,
            from_email: None,
            smtp_server: None,
            smtp_user: None,
            smtp_port: None,
            smtp_encryption: None,
            cv_skel: None,
        }
    }

    pub fn editor(&self) -> Option<&str> {
        self.editor.as_ref().map(String::as_str)
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.editor = Some(String::from(editor));
    }

    pub fn send_command(&self) -> Option<&str> {
        self.send_command.as_ref().map(String::as_str)
    }

    pub fn set_send_command(&mut self, send_command: Option<&str>) {
        self.send_command = send_command.map(String::from);
    }

    pub fn from_email(&self) -> Option<&str> {
        self.from_email.as_ref().map(String::as_str)
    }

    pub fn set_from_email(&mut self, from_email: &str) {
        self.from_email = Some(String::from(from_email));
    }

    pub fn smtp_server(&self) -> Option<&str> {
        self.smtp_server.as_ref().map(String::as_str)
    }

    pub fn set_smtp_server(&mut self, smtp_server: Option<&str>) {
        self.smtp_server = smtp_server.map(String::from);
    }

    pub fn smtp_user(&self) -> Option<&str> {
        self.smtp_user.as_ref().map(String::as_str)
    }

    pub fn set_smtp_user(&mut self, smtp_user: Option<&str>) {
        self.smtp_user = smtp_user.map(String::from);
    }

    pub fn smtp_port(&self) -> Option<u16> {
        self.smtp_port
    }

    pub fn set_smtp_port(&mut self, smtp_port: Option<u16>) {
        self.smtp_port = smtp_port;
    }

    pub fn smtp_encryption(&self) -> Option<&str> {
        self.smtp_encryption.as_ref().map(String::as_str)
    }

    pub fn set_smtp_encryption(&mut self, smtp_encryption: Option<&str>) {
        self.smtp_encryption = smtp_encryption.map(String::from);
    }

    pub fn cv_skel(&self) -> Option<&str> {
        self.cv_skel.as_ref().map(String::as_str)
    }

    pub fn set_cv_skel(&mut self, cv_skel: Option<&str>) {
        self.cv_skel = cv_skel.map(String::from);
    }
}

impl RootFile {
    pub fn new() -> Self {
        Self {
            version: String::from(PROJECT_VERSION),
            user_config: UserConfig::new(),
            repos: Vec::new(),
            lists: HashMap::new(),
        }
    }

    pub fn find_repo_by_path(&self, path: &str) -> Option<RepoMetadata> {
        let root_path = find_repo_root(path)?.to_string_lossy().to_string();
        self.repos.iter().find(|r| r.path() == root_path).cloned()
    }

    // TODO: use a hash table instead?
    // The number of repos should not make the lookup slow
    fn repo_exists(&self, name: &str, path: &str) -> bool {
        self.repos
            .iter()
            .find(|r| r.name() == name || r.path() == path)
            .is_some()
    }

    pub fn register_repo(
        &mut self, name: &str, path: &str,
    ) -> Result<&RepoMetadata, UserDataError> {
        if self.repo_exists(name, path) {
            return Err(UserDataError::new(UserDataErrorCode::RepoAlreadyExists));
        }
        let Some(path) = find_repo_root(path) else {
            return Err(UserDataError::new(UserDataErrorCode::NotAGitRepo));
        };
        let repo = RepoMetadata::new(name, path.to_string_lossy().to_string().as_str());
        self.repos.push(repo);
        Ok(self.repos.last().unwrap())
    }

    pub fn delete_repo(&mut self, name: &str) -> Result<(), UserDataError> {
        let count = self.repos.len();
        self.repos.retain(|r| r.name() != name);
        if count == self.repos.len() {
            return Err(UserDataError::new(UserDataErrorCode::RepoDoesNotExist));
        }
        Ok(())
    }

    pub fn repos(&self) -> &[RepoMetadata] {
        self.repos.as_slice()
    }

    pub fn config(&self) -> &UserConfig {
        &self.user_config
    }

    pub fn config_mut(&mut self) -> &mut UserConfig {
        &mut self.user_config
    }

    pub fn add_mailing_list(&mut self, name: &str, email: &str) -> Result<(), UserDataError> {
        if self.lists.contains_key(name) {
            return Err(UserDataError::new(UserDataErrorCode::ListAlreadyExists));
        }
        if let Some(list) = MailingList::new(name, email) {
            self.lists.insert(String::from(name), list);
            Ok(())
        } else {
            Err(UserDataError::new_with_message(
                UserDataErrorCode::InputError,
                format!(
                    "The list cannot be created, the address format must \
                     be valid and the name must be an alphanumeric string"
                ),
            ))
        }
    }

    pub fn delete_mailing_list(&mut self, name: &str) -> Result<(), UserDataError> {
        match self.lists.remove(name) {
            Some(_) => Ok(()),
            None => Err(UserDataError::new_with_message(
                UserDataErrorCode::ListDoesNotExist,
                format!("List {} is not known", name),
            )),
        }
    }

    pub fn find_mailing_list(&self, name : &str) -> Option<&MailingList> {
        self.lists.get(name)
    }

    pub fn version(&self) -> &str {
        self.version.as_str()
    }
}
