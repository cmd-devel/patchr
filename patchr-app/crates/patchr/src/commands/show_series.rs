use std::ops::ControlFlow;

use log::debug;

use crate::cli_print;
use crate::{cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, CommandBuilderErrorCode, SHOW_SERIES};

const VERBOSE_FLAG: &str = "v";

pub struct ShowSeries {
    series_name: String,
    verbose: bool,
}
pub struct ShowSeriesBuilder {
    series_name: Option<String>,
    verbose: bool,
}

impl ShowSeries {
    fn new(series_name: &str, verbose: bool) -> Self {
        ShowSeries {
            series_name: String::from(series_name),
            verbose,
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(ShowSeriesBuilder::new())
    }
}

impl ShowSeriesBuilder {
    fn new() -> Self {
        Self {
            series_name: None,
            verbose: false,
        }
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

        if self.verbose {
            cli_print!("{:?}", series)
        } else {
            cli_print!("{}", series);
        }
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

    fn add_flag(&mut self, flag: &str) -> Result<(), CommandBuilderError> {
        if flag == VERBOSE_FLAG {
            self.verbose = true;
            return Ok(());
        };

        Err(CommandBuilderError::new(
            CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn requires_value(&self, flag: &str) -> Result<bool, CommandBuilderError> {
        if flag == VERBOSE_FLAG {
            return Ok(false);
        }
        Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn name(&self) -> &str {
        SHOW_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let Some(series_name) = self.series_name.as_ref() {
            Ok(Box::new(ShowSeries::new(series_name.as_str(), self.verbose)))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing series name"),
            ))
        }
    }
}
