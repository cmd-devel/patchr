use std::ops::ControlFlow;

use log::debug;

use crate::{
    cli_print_error, commands::common::edit_in_text_editor, get_repo_mut_or_fail, user_data::user_data::UserData
};

use super::{Command, CommandBuilder, CommandBuilderError, EDIT_SERIES};

#[derive(Clone, Copy)]
enum EditSeriesTarget {
    Name,
    Title,
    Cv,
    ShortName,
    Cc,
}

pub struct EditSeries {
    target: EditSeriesTarget,
    series_name: String,
}

pub struct EditSeriesBuilder {
    target: Option<EditSeriesTarget>,
    series_name: Option<String>,
}

impl TryFrom<&str> for EditSeriesTarget {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "name" => Ok(EditSeriesTarget::Name),
            "title" => Ok(EditSeriesTarget::Title),
            "cv" => Ok(EditSeriesTarget::Cv),
            "short" => Ok(EditSeriesTarget::ShortName),
            "cc" => Ok(EditSeriesTarget::Cc),
            _ => Err(()),
        }
    }
}

impl EditSeries {
    fn new(target: EditSeriesTarget, series_name: &str) -> Self {
        EditSeries {
            target,
            series_name: String::from(series_name),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(EditSeriesBuilder::new())
    }
}

impl EditSeriesBuilder {
    fn new() -> Self {
        Self {
            target: None,
            series_name: None,
        }
    }
}

impl Command for EditSeries {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Edit series");

        let user_config = user_data.config().clone();
        let repo = get_repo_mut_or_fail!(user_data);

        let Some(series) = repo
            .repo_mut()
            .get_series_by_name_mut(self.series_name.as_str())
        else {
            cli_print_error!("Unknown series : {}", self.series_name.as_str());
            return ControlFlow::Break(());
        };

        let content = match self.target {
            EditSeriesTarget::Name => series.name(),
            EditSeriesTarget::Title => series.title(),
            EditSeriesTarget::Cv => series.cover_letter(),
            EditSeriesTarget::ShortName => series.short_name(),
            EditSeriesTarget::Cc => series.cc(),
        };

        let Some(new_content) = edit_in_text_editor(&user_config, content) else {
            return ControlFlow::Break(());
        };

        let update_res = match self.target {
            EditSeriesTarget::Name => series.set_name(new_content.as_str()),
            EditSeriesTarget::Title => series.set_title(new_content.as_str()),
            EditSeriesTarget::Cv => series.set_cover_letter(new_content.as_str()),
            EditSeriesTarget::ShortName => series.set_short_name(new_content.as_str()),
            EditSeriesTarget::Cc => series.set_cc(new_content.as_str()),
        };

        match update_res {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => {
                cli_print_error!("Failed to update the series, {}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl CommandBuilder for EditSeriesBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.target.is_none() {
            if let Ok(target) = EditSeriesTarget::try_from(value) {
                self.target = Some(target);
                return Ok(());
            }
            return Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::UnexpectedValue,
                format!("Integer value expected, found '{}'", value),
            ));
        }

        if self.series_name.is_none() {
            self.series_name = Some(String::from(value));
            return Ok(());
        }

        Err(CommandBuilderError::unexpected_value(value))
    }

    fn name(&self) -> &str {
        EDIT_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(target), Some(series_name)) = (self.target, &self.series_name) {
            Ok(Box::new(EditSeries::new(target, series_name.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing arguments"),
            ))
        }
    }
}
