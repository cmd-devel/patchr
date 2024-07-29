use std::ops::ControlFlow;

use log::debug;

use crate::{cli_print, get_repo_mut_or_fail};
use crate::{cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, ADD_REVISION};

pub struct AddRevision {
    series_name: String,
}
pub struct AddRevisionBuilder {
    series_name: Option<String>,
}

impl AddRevision {
    fn new(series_name: &str) -> Self {
        AddRevision {
            series_name: String::from(series_name),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(AddRevisionBuilder::new())
    }
}

impl AddRevisionBuilder {
    fn new() -> Self {
        Self { series_name: None }
    }
}

impl Command for AddRevision {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Add revision to {}", self.series_name);
        let repo = get_repo_mut_or_fail!(user_data);

        let Some(series) = repo
            .repo_mut()
            .get_series_by_name_mut(self.series_name.as_str())
        else {
            cli_print_error!("Error, unknown series : {}", self.series_name.as_str());
            return ControlFlow::Break(());
        };

        series.add_revision();
        cli_print!("Revision added: v{}", series.current_revision());
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for AddRevisionBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.series_name.is_some() {
            Err(CommandBuilderError::unexpected_value(value))
        } else {
            self.series_name = Some(String::from(value));
            Ok(())
        }
    }

    fn name(&self) -> &str {
        ADD_REVISION
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let Some(series_name) = self.series_name.as_ref() {
            Ok(Box::new(AddRevision::new(series_name.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing series name"),
            ))
        }
    }
}
