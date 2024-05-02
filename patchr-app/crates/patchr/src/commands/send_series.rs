use std::ops::ControlFlow;

use common::util::rust::result_to_control_flow;
use git::patch_sender::{GitPatchSender, PatchSender};
use log::{debug, trace};

use crate::{
    cli_print_error,
    user_data::user_data::{root_tmp_dir_path, UserData},
};

use super::{Command, CommandBuilder, CommandBuilderError, SEND_SERIES};

const CC_FLAG: &str = "c";

pub struct SendSeries {
    series_name: String,
    first_commit: String,
    last_commit: String,
    to_email: String,
    cc: Option<String>,
}

pub struct SendSeriesBuilder {
    series_name: Option<String>,
    first_commit: Option<String>,
    last_commit: Option<String>,
    to_email: Option<String>,
    cc: Option<String>,
}

impl SendSeries {
    fn new(
        series_name: &str, first_commit: &str, last_commit: &str, to_email: &str, cc: Option<&str>,
    ) -> Self {
        SendSeries {
            series_name: String::from(series_name),
            first_commit: String::from(first_commit),
            last_commit: String::from(last_commit),
            to_email: String::from(to_email),
            cc: cc.map(|c| String::from(c)),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(SendSeriesBuilder::new())
    }
}

impl SendSeriesBuilder {
    fn new() -> Self {
        Self {
            series_name: None,
            first_commit: None,
            last_commit: None,
            to_email: None,
            cc: None,
        }
    }
}

impl Command for SendSeries {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Send series");

        let user_config = user_data.config().clone();

        let Some(repo) = user_data.repo() else {
            trace!("cannot get the current repo");
            return ControlFlow::Break(());
        };

        let Some(series) = repo.repo().get_series_by_name(self.series_name.as_str()) else {
            cli_print_error!("Unknown series : {}", self.series_name.as_str());
            return ControlFlow::Break(());
        };

        let Some(from_email) = user_config.from_email() else {
            cli_print_error!("Missing source email");
            return ControlFlow::Break(());
        };

        let mut sender_builder = GitPatchSender::builder(from_email);
        
        if let Some(send_command) = user_config.send_command() {
            sender_builder.set_send_command(send_command);
        };
        if let Some(smtp_server) = user_config.smtp_server() {
            sender_builder.set_smtp_server(smtp_server);
        };
        if let Some(smtp_port) = user_config.smtp_port() {
            sender_builder.set_smtp_port(smtp_port);
        };
        if let Some(smtp_user) = user_config.smtp_user() {
            sender_builder.set_smtp_user(smtp_user);
        };
        if let Some(smtp_encryption) = user_config.smtp_encryption() {
            sender_builder.set_smtp_encryption(smtp_encryption);
        };
        
        let sender = sender_builder.build();
        let rtmp = result_to_control_flow(root_tmp_dir_path(), |e| {
            cli_print_error!("{}", e.to_string());
            ()
        })?;
        let send_res = sender.send(
            series,
            self.to_email.as_str(),
            &rtmp,
            self.first_commit.as_str(),
            self.last_commit.as_str(),
            self.cc.as_deref(),
        );

        match send_res {
            Ok(_) => ControlFlow::Continue(()),
            Err(e) => {
                cli_print_error!("Failed to send the series, {}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl CommandBuilder for SendSeriesBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.series_name.is_none() {
            self.series_name = Some(String::from(value));
            return Ok(());
        }

        if self.first_commit.is_none() {
            self.first_commit = Some(String::from(value));
            return Ok(());
        }

        if self.last_commit.is_none() {
            self.last_commit = Some(String::from(value));
            return Ok(());
        }

        if self.to_email.is_none() {
            self.to_email = Some(String::from(value));
            return Ok(());
        }

        return Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnexpectedValue,
            String::from(value),
        ));
    }

    fn add_flag_and_value(&mut self, flag: &str, value: &str) -> Result<(), CommandBuilderError> {
        if flag == CC_FLAG {
            let value = value.trim();
            self.cc = Some(String::from(value));
            return Ok(());
        }
        Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn requires_value(&self, flag: &str) -> Result<bool, CommandBuilderError> {
        if flag == CC_FLAG {
            return Ok(true);
        }
        Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn name(&self) -> &str {
        SEND_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(series_name), Some(first_commit), Some(last_commit), Some(to_email)) =
            (&self.series_name, &self.first_commit, &self.last_commit, &self.to_email)
        {
            Ok(Box::new(SendSeries::new(
                series_name.as_str(),
                first_commit.as_str(),
                last_commit.as_str(),
                to_email.as_str(),
                self.cc.as_deref(),
            )))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing arguments"),
            ))
        }
    }
}
