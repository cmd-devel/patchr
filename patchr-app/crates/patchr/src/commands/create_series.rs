use std::ops::ControlFlow;

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
        let repo = get_repo_mut_or_fail!(user_data);

        match repo
            .repo_mut()
            .add_series(self.name.as_str(), self.title.as_str())
        {
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

        Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnexpectedValue,
            String::from(value),
        ))
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
