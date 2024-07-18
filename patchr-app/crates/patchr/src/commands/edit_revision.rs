use std::ops::ControlFlow;

use log::debug;

use crate::{
    cli_print_error, commands::common::edit_in_text_editor, user_data::user_data::UserData,
};

use super::{Command, CommandBuilder, CommandBuilderError, EDIT_REVISION};

pub struct EditRevision {
    series_name: String,
    revision: usize,
}
pub struct EditRevisionBuilder {
    series_name: Option<String>,
    revision: Option<usize>,
}

impl EditRevision {
    fn new(series_name: &str, revision: usize) -> Self {
        EditRevision {
            series_name: String::from(series_name),
            revision,
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(EditRevisionBuilder::new())
    }
}

impl EditRevisionBuilder {
    fn new() -> Self {
        Self {
            series_name: None,
            revision: None,
        }
    }
}

impl Command for EditRevision {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Edit revision");

        let user_config = user_data.config().clone();

        let Some(repo) = user_data.repo_mut() else {
            // use a function to factor that
            cli_print_error!("Not in a repo");
            return ControlFlow::Break(());
        };

        let Some(series) = repo
            .repo_mut()
            .get_series_by_name_mut(self.series_name.as_str())
        else {
            cli_print_error!("Unknown series : {}", self.series_name.as_str());
            return ControlFlow::Break(());
        };

        let Some(revision) = series.revision_mut(self.revision) else {
            cli_print_error!("Unknown revision : {}", self.revision);
            return ControlFlow::Break(());
        };

        let Some(new_content) = edit_in_text_editor(&user_config, revision.content()) else {
            return ControlFlow::Break(());
        };

        revision.set_content(new_content.as_str());
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for EditRevisionBuilder {
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
        EDIT_REVISION
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(series_name), Some(revision)) = (self.series_name.as_ref(), self.revision) {
            Ok(Box::new(EditRevision::new(series_name.as_str(), revision)))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing arguments"),
            ))
        }
    }
}
