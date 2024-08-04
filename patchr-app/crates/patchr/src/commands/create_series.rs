use std::ops::ControlFlow;

use git::series::Series;
use log::debug;

use crate::{cli_print, cli_print_error, get_repo_mut_or_fail, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, CREATE_SERIES};

pub struct CreateSeries {
    name: String,
    title: String,
}
pub struct CreateSeriesBuilder {
    name: Option<String>,
    title: Option<String>,
}

impl CreateSeries {
    fn new(name: &str, title: &str) -> Self {
        CreateSeries {
            name: String::from(name),
            title: String::from(title),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(CreateSeriesBuilder::new())
    }
}

impl CreateSeriesBuilder {
    fn new() -> Self {
        Self {
            name: None,
            title: None,
        }
    }
}

impl Command for CreateSeries {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Create series");

        // Copy the string so that we don't borrow user_data for too long
        let cv_skel = user_data.config().cv_skel().map(String::from);

        let repo = get_repo_mut_or_fail!(user_data);

        let repo_dirname = repo.meta().dirname();
        let short_name = Series::validate_short_name(repo_dirname.as_str());

        if short_name.is_none() {
            cli_print!("The repo name cannot be used as a short name")
        }
        match repo.repo_mut().add_series(
            self.name.as_str(),
            self.title.as_str(),
            short_name,
            cv_skel.as_ref().map(String::as_str),
        ) {
            Ok(_) => {
                cli_print!("Series created");
                ControlFlow::Continue(())
            }
            Err(e) => {
                cli_print!("Failed to create series");
                cli_print_error!("{}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl CommandBuilder for CreateSeriesBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.name.is_none() {
            self.name = Some(String::from(value));
            return Ok(());
        }

        if self.title.is_none() {
            self.title = Some(String::from(value));
            return Ok(());
        }

        Err(CommandBuilderError::unexpected_value(value))
    }

    fn name(&self) -> &str {
        CREATE_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(name), Some(title)) = (&self.name, &self.title) {
            Ok(Box::new(CreateSeries::new(name.as_str(), title.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing arguments"),
            ))
        }
    }
}
