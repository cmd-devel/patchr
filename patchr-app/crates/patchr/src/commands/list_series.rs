use std::ops::ControlFlow;

use log::debug;

use crate::{cli_print, cli_print_error, get_repo_or_fail, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, LIST_SERIES};

pub struct ListSeries {}
pub struct ListSeriesBuilder {}

impl ListSeries {
    fn new() -> Self {
        ListSeries {}
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(ListSeriesBuilder::new())
    }
}

impl ListSeriesBuilder {
    fn new() -> Self {
        Self {}
    }
}

impl Command for ListSeries {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("List series");

        let repo = get_repo_or_fail!(user_data);
        repo.repo().series().iter().for_each(|s| {
            cli_print!("- {} (v{})", s.name(), s.current_revision());
        });
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for ListSeriesBuilder {
    fn name(&self) -> &str {
        LIST_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        Ok(Box::new(ListSeries::new()))
    }
}
