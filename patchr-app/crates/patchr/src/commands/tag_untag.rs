use std::marker::PhantomData;
use std::ops::ControlFlow;

use git::util::CommitId;
use ::git::util::{Commit, CommitTag, GitRepo};
use log::debug;

use crate::{cli_print, get_repo_or_fail, open_git_repo_or_fail};
use crate::{cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, TAG, UNTAG};

pub struct Tag {
    tag: CommitTag,
    value: String,
    commit: String,
}

pub struct UnTag {
    tag: CommitTag,
    value: Option<String>,
    commit: String,
}

trait TaggingCommand: Command {
    fn name() -> &'static str;
    fn new<C: TaggingCommand>(builder: &TaggingBuilder<C>) -> Result<Box<dyn Command>, CommandBuilderError>;
}

struct TaggingBuilder<C: TaggingCommand> {
    tag: Option<CommitTag>,
    value: Option<String>,
    commmit: Option<String>,
    phantom: PhantomData<C>,
}

impl<C: TaggingCommand> TaggingBuilder<C> {
    fn new() -> Self {
        Self {
            tag: None,
            value: None,
            commmit: None,
            phantom: PhantomData,
        }
    }
}

impl Tag {
    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(TaggingBuilder::<Tag>::new())
    }
}

impl UnTag {
    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(TaggingBuilder::<UnTag>::new())
    }
}

impl TaggingCommand for UnTag {
    fn name() -> &'static str {
        UNTAG
    }

    fn new<C: TaggingCommand>(builder: &TaggingBuilder<C>) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(tag), Some(commit)) = (&builder.tag, &builder.commmit) {
            Ok(Box::new(Self {
                tag: tag.clone(),
                value: builder.value.clone(),
                commit: commit.clone(),
            }))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing values"),
            ))
        }
    }
}

impl TaggingCommand for Tag {
    fn name() -> &'static str {
        TAG
    }

    fn new<C: TaggingCommand>(builder: &TaggingBuilder<C>) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(tag), Some(value), Some(commit)) = (&builder.tag, &builder.value, &builder.commmit) {
            Ok(Box::new(Self {
                tag: tag.clone(),
                value: value.clone(),
                commit: commit.clone(),
            }))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing values"),
            ))
        }
    }
}

fn find_commit<'a>(repo: &'a GitRepo, commit: &str) -> Option<Commit<'a>> {
    let commit_id = match CommitId::new(commit) {
        Ok(c) => c,
        Err(e) => {
            cli_print_error!("Failed to decode the commit hash: {}", e);
            return None;
        }
    };
    match repo.find_commit(&commit_id) {
        Ok(c) => Some(c),
        Err(e) => {
            cli_print_error!("{}", e);
            None
        }
    }
}

impl Command for Tag {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("tag : {} {} {}", self.commit, self.tag, self.value);

        let repo = get_repo_or_fail!(user_data);
        let git_repo = open_git_repo_or_fail!(repo);
        let commit = match find_commit(&git_repo, self.commit.as_str()) {
            Some(c) => c,
            None => return ControlFlow::Break(())
        };

        match commit.add_tag(&self.tag, Some(self.value.as_str())) {
            Ok(id) => {
                cli_print!("Tag added");
                cli_print!("New commit: {}", id);
                ControlFlow::Continue(())
            },
            Err(e) => {
                cli_print_error!("Failed to add the tag: {}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl Command for UnTag {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        // TODO: implement display and add a debug trace
        let repo = get_repo_or_fail!(user_data);
        let git_repo = open_git_repo_or_fail!(repo);
        let commit = match find_commit(&git_repo, self.commit.as_str()) {
            Some(c) => c,
            None => return ControlFlow::Break(())
        };
        let result = match &self.value {
            Some(v) => {
                commit.remove_tag(&self.tag, Some(v))
            }
            None => {
                commit.remove_tag_all(&self.tag)
            }
        };

        match result {
            Ok(id) => {
                cli_print!("Tag removed");
                cli_print!("New commit: {}", id);
                ControlFlow::Continue(())
            },
            Err(e) => {
                cli_print_error!("Failed to remove the tag: {}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl<C: TaggingCommand> CommandBuilder for TaggingBuilder<C> {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.commmit.is_none() {
            self.commmit = Some(String::from(value));
            return Ok(());
        }

        if self.tag.is_none() {
            self.tag = Some(CommitTag::from(value));
            return Ok(());
        }

        if self.value.is_none() {
            self.value = Some(String::from(value));
            return Ok(());
        }

        Err(CommandBuilderError::unexpected_value(value))
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        C::new(self)
    }

    fn name(&self) -> &str {
        C::name()
    }
}