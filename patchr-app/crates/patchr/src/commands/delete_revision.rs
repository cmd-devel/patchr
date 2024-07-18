use std::ops::ControlFlow;

use log::debug;

use crate::{cli_print_error, get_repo_mut_or_fail, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, DELETE_REVISION};

pub struct DeleteRevision {
    series_name: String,
    revision: usize,
}
pub struct DeleteRevisionBuilder {
    series_name: Option<String>,
    revision: Option<usize>,
}

impl DeleteRevision {
    fn new(series_name: &str, revision: usize) -> Self {
        DeleteRevision {
            series_name: String::from(series_name),
            revision,
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(DeleteRevisionBuilder::new())
    }
}

impl DeleteRevisionBuilder {
    fn new() -> Self {
        Self {
            series_name: None,
            revision: None,
        }
    }
}

impl Command for DeleteRevision {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Delete revision {} from {}", self.revision, self.series_name);
        let repo = get_repo_mut_or_fail!(user_data);

        let Some(series) = repo
            .repo_mut()
            .get_series_by_name_mut(self.series_name.as_str())
        else {
            cli_print_error!("Unknown series : {}", self.series_name.as_str());
            return ControlFlow::Break(());
        };

        series.delete_revision(self.revision);
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for DeleteRevisionBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.series_name.is_none() {
            self.series_name = Some(String::from(value));
            return Ok(());
        }

        if self.revision.is_none() {
            if let Ok(parsed_value) = value.parse::<usize>() {
                self.revision = Some(parsed_value);
                return Ok(());
            } else {
                return Err(CommandBuilderError::new(
                    super::CommandBuilderErrorCode::UnexpectedValue,
                    String::from(value),
                ));
            }
        }
        Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnexpectedValue,
            String::from(value),
        ))
    }

    fn name(&self) -> &str {
        DELETE_REVISION
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(series_name), Some(revision)) = (self.series_name.as_ref(), self.revision) {
            Ok(Box::new(DeleteRevision::new(series_name.as_str(), revision)))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing arguments"),
            ))
        }
    }
}
