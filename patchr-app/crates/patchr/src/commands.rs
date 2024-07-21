use std::ops::ControlFlow;

pub mod add_mailing_list;
pub mod add_revision;
pub mod cleantmp;
mod common;
pub mod configure;
pub mod create_series;
pub mod delete_mailing_list;
pub mod delete_repo;
pub mod delete_revision;
pub mod delete_series;
pub mod edit_revision;
pub mod edit_series;
pub mod help;
pub mod list_repos;
pub mod list_series;
pub mod register_repo;
pub mod send_series;
pub mod set_verbose;
pub mod show_series;

use cleantmp::CleanTmp;

use crate::user_data::user_data::UserData;

use self::{
    add_mailing_list::AddMailingList, add_revision::AddRevision, configure::Configure,
    create_series::CreateSeries, delete_mailing_list::DeleteMailingList, delete_repo::DeleteRepo,
    delete_revision::DeleteRevision, delete_series::DeleteSeries, edit_revision::EditRevision,
    edit_series::EditSeries, list_repos::ListRepos, list_series::ListSeries,
    register_repo::RegisterRepo, send_series::SendSeries, show_series::ShowSeries,
};

macro_rules! declare_flag {
    ($name:ident, $letter:ident) => {
        pub const $name: &str = stringify!($letter);
    };
}

macro_rules! declare_command {
    ($name:ident, $mnemonic:ident) => {
        pub const $name: &str = stringify!($mnemonic);
    };
}

// Global flags
declare_flag!(VERBOSE, v);
declare_flag!(HELP, h);

// Commands
declare_command!(LIST_SERIES, list);
declare_command!(REGISTER_REPO, register);
declare_command!(DELETE_REPO, delrepo);
declare_command!(LIST_REPOS, repos);
declare_command!(CREATE_SERIES, create);
declare_command!(DELETE_SERIES, delete);
declare_command!(EDIT_SERIES, edit);
declare_command!(CONFIGURE, config);
declare_command!(ADD_REVISION, addrev);
declare_command!(DELETE_REVISION, delrev);
declare_command!(EDIT_REVISION, editrev);
declare_command!(SEND_SERIES, send);
declare_command!(SHOW_SERIES, show);
declare_command!(ADD_LIST, addlist);
declare_command!(DELETE_LIST, dellist);
declare_command!(CLEAN_TMP, cleantmp);

pub trait Command {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()>;
}

#[derive(Debug, Clone)]
pub enum CommandBuilderErrorCode {
    UnknownFlag,
    UnexpectedValue,
    MissingValue,
    IncompatibleValues,
    InvalidValues,
}

#[derive(Debug, Clone)]
pub struct CommandBuilderError {
    code: CommandBuilderErrorCode,
    message: String,
}

pub trait CommandBuilder {
    fn add_flag(&mut self, flag: &str) -> Result<(), CommandBuilderError> {
        Err(CommandBuilderError::new(
            CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        Err(CommandBuilderError::unexpected_value(value))
    }

    fn add_flag_and_value(&mut self, flag: &str, _value: &str) -> Result<(), CommandBuilderError> {
        Err(CommandBuilderError::new(
            CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn requires_value(&self, flag: &str) -> Result<bool, CommandBuilderError> {
        Err(CommandBuilderError::new(
            CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn name(&self) -> &str;

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError>;
}

impl CommandBuilderError {
    pub fn new(code: CommandBuilderErrorCode, message: String) -> Self {
        Self { code, message }
    }

    pub fn code(&self) -> CommandBuilderErrorCode {
        self.code.clone()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn message_move(self) -> String {
        self.message
    }

    pub fn unexpected_value(value: &str) -> Self {
        CommandBuilderError::new(
            CommandBuilderErrorCode::UnexpectedValue,
            String::from(value),
        )
    }
}

pub fn get_command_builder(name: &str) -> Option<Box<dyn CommandBuilder>> {
    match name {
        LIST_SERIES => Some(ListSeries::builder()),
        REGISTER_REPO => Some(RegisterRepo::builder()),
        DELETE_REPO => Some(DeleteRepo::builder()),
        LIST_REPOS => Some(ListRepos::builder()),
        CREATE_SERIES => Some(CreateSeries::builder()),
        DELETE_SERIES => Some(DeleteSeries::builder()),
        EDIT_SERIES => Some(EditSeries::builder()),
        CONFIGURE => Some(Configure::builder()),
        ADD_REVISION => Some(AddRevision::builder()),
        DELETE_REVISION => Some(DeleteRevision::builder()),
        EDIT_REVISION => Some(EditRevision::builder()),
        SEND_SERIES => Some(SendSeries::builder()),
        SHOW_SERIES => Some(ShowSeries::builder()),
        ADD_LIST => Some(AddMailingList::builder()),
        DELETE_LIST => Some(DeleteMailingList::builder()),
        CLEAN_TMP => Some(CleanTmp::builder()),
        _ => None,
    }
}
