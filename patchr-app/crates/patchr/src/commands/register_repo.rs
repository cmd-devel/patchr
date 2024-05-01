use std::{env, ops::ControlFlow};

use common::util::misc::LINE_SEP;

use log::debug;

use crate::{cli_print, cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, REGISTER_REPO};

pub struct RegisterRepo {
    name: String,
}
pub struct RegisterRepoBuilder {
    name: Option<String>,
}

impl RegisterRepo {
    fn new(name: &str) -> Self {
        RegisterRepo {
            name: String::from(name),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(RegisterRepoBuilder::new())
    }
}

impl RegisterRepoBuilder {
    fn new() -> Self {
        Self { name: None }
    }
}

impl Command for RegisterRepo {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Registering repo : {}", self.name);
        match env::current_dir() {
            Ok(path) => {
                let path_str = path.to_string_lossy().to_string();
                if let Err(e) = user_data.register_repo(self.name.as_str(), path_str.as_str()) {
                    cli_print_error!("Failed to register repo : {}", e.to_string());
                    return ControlFlow::Break(());
                }
                cli_print!(
                    "Repo added{}name: {}{}directory: {}",
                    LINE_SEP,
                    self.name.as_str(),
                    LINE_SEP,
                    path_str.as_str()
                );
                ControlFlow::Continue(())
            }
            Err(e) => {
                cli_print_error!("Cannot get the current directory : {}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl CommandBuilder for RegisterRepoBuilder {
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
        REGISTER_REPO
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let Some(name) = self.name.as_ref() {
            Ok(Box::new(RegisterRepo::new(name.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing repo name"),
            ))
        }
    }
}
