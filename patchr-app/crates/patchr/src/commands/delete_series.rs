use std::ops::ControlFlow;

use log::debug;

use crate::{cli_print, cli_print_error, get_repo_mut_or_fail, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, DELETE_SERIES};

pub struct DeleteSeries {
    name: String,
}
pub struct DeleteSeriesBuilder {
    name: Option<String>,
}

impl DeleteSeries {
    fn new(name: &str) -> Self {
        DeleteSeries {
            name: String::from(name),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(DeleteSeriesBuilder::new())
    }
}

impl DeleteSeriesBuilder {
    fn new() -> Self {
        Self { name: None }
    }
}

impl Command for DeleteSeries {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Delete series");
        let repo = get_repo_mut_or_fail!(user_data);

        match repo.repo_mut().delete_series(self.name.as_str()) {
            Ok(_) => {
                cli_print!("Series deleted");
                ControlFlow::Continue(())
            }
            Err(e) => {
                cli_print!("Failed to delete series");
                cli_print_error!("{}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl CommandBuilder for DeleteSeriesBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.name.is_none() {
            self.name = Some(String::from(value));
            return Ok(());
        }

        Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnexpectedValue,
            String::from(value),
        ))
    }

    fn name(&self) -> &str {
        DELETE_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let Some(name) = &self.name {
            Ok(Box::new(DeleteSeries::new(name.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing series name"),
            ))
        }
    }
}
