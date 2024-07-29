use std::ops::ControlFlow;

use colored::{ColoredString, Colorize};
use log::debug;

use crate::{cli_print, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, LIST_REPOS};

pub struct ListRepos {}
pub struct ListReposBuilder;

impl ListRepos {
    fn new() -> Self {
        ListRepos {}
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(ListReposBuilder::new())
    }
}

impl ListReposBuilder {
    fn new() -> Self {
        Self {}
    }
}

impl Command for ListRepos {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("List repos");
        let current_repo = user_data.repo();
        user_data.repos().iter().for_each(|r| {
            let mut line = ColoredString::from(format!("- {} : {}", r.name(), r.path()));
            if current_repo.is_some_and(|cr| cr.meta() == r) {
                line = line.green();
            }
            cli_print!("{}", line);
        });
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for ListReposBuilder {
    fn name(&self) -> &str {
        LIST_REPOS
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        Ok(Box::new(ListRepos::new()))
    }
}
