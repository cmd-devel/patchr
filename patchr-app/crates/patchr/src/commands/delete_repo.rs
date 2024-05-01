use std::{fs, ops::ControlFlow};

use common::util::rust::result_to_control_flow;
use log::debug;

use crate::{cli_print, cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, DELETE_REPO};

pub struct DeleteRepo {
    name: String,
}
pub struct DeleteRepoBuilder {
    name: Option<String>,
}

impl DeleteRepo {
    fn new(name: &str) -> Self {
        DeleteRepo {
            name: String::from(name),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(DeleteRepoBuilder::new())
    }
}

impl DeleteRepoBuilder {
    fn new() -> Self {
        Self { name: None }
    }
}

impl Command for DeleteRepo {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Delete repo : {}", self.name);
        let data_path = {
            let Some(repo) = user_data.repo() else {
                return ControlFlow::Break(());
            };
            repo.meta().path().to_owned()
        };
        result_to_control_flow(user_data.delete_repo(self.name.as_str()), |e| {
            cli_print_error!("{}", e);
            ()
        })?;
        debug!("Delete repo data : {}", &data_path);
        let _ = fs::remove_file(&data_path);
        cli_print!("Repo deleted");
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for DeleteRepoBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.name.is_some() {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::UnexpectedValue,
                String::from(value),
            ))
        } else {
            self.name = Some(String::from(value));
            Ok(())
        }
    }

    fn name(&self) -> &str {
        DELETE_REPO
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let Some(name) = self.name.as_ref() {
            Ok(Box::new(DeleteRepo::new(name.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing repo name"),
            ))
        }
    }
}
