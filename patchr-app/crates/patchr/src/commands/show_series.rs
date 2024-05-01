use std::ops::ControlFlow;

use log::debug;

use crate::cli_print;
use crate::{cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, SHOW_SERIES};

pub struct ShowSeries {
    series_name: String,
}
pub struct ShowSeriesBuilder {
    series_name: Option<String>,
}

impl ShowSeries {
    fn new(series_name: &str) -> Self {
        ShowSeries {
            series_name: String::from(series_name),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(ShowSeriesBuilder::new())
    }
}

impl ShowSeriesBuilder {
    fn new() -> Self {
        Self { series_name: None }
    }
}

impl Command for ShowSeries {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Show series : {}", self.series_name);
        let Some(repo) = user_data.repo_mut() else {
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

        cli_print!("{}", series);
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for ShowSeriesBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.series_name.is_some() {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::UnexpectedValue,
                String::from(value),
            ))
        } else {
            self.series_name = Some(String::from(value));
            Ok(())
        }
    }

    fn name(&self) -> &str {
        SHOW_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let Some(series_name) = self.series_name.as_ref() {
            Ok(Box::new(ShowSeries::new(series_name.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing series name"),
            ))
        }
    }
}
