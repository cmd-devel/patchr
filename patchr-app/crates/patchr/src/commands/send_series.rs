use std::{io, ops::ControlFlow};

use common::util::{misc::LINE_SEP, rust::result_to_control_flow};
use git::{
    patch_sender::{GitPatchSender, PatchSender},
    repo::RepoData,
    series::SeriesLog,
};
use log::{debug, trace};

use crate::{
    cli_print, cli_print_error,
    user_data::user_data::{root_tmp_dir_path, UserData},
};

use super::{Command, CommandBuilder, CommandBuilderError, SEND_SERIES};

const CC_FLAG: &str = "c";
const INTERACTIVE_FLAG: &str = "i";

const YES_KEY: &str = "y";

pub struct SendSeries {
    series_name: String,
    first_commit: Option<String>,
    last_commit: Option<String>,
    to_email: String,
    cc: Option<String>,
    interactive: bool,
}

pub struct SendSeriesBuilder {
    series_name: Option<String>,
    first_commit: Option<String>,
    last_commit: Option<String>,
    to_email: Option<String>,
    cc: Option<String>,
    interactive: bool,
}

impl SendSeries {
    fn new(series_name: &str, to_email: &str, cc: Option<&str>, interactive: bool) -> Self {
        SendSeries {
            series_name: String::from(series_name),
            first_commit: None,
            last_commit: None,
            to_email: String::from(to_email),
            cc: cc.map(|c| String::from(c)),
            interactive,
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(SendSeriesBuilder::new())
    }

    fn set_commit_range(&mut self, first_commit: &str, last_commit: &str) {
        self.first_commit = Some(String::from(first_commit));
        self.last_commit = Some(String::from(last_commit));
    }

    fn get_to_email<'a>(&'a self, user_data: &'a UserData) -> &'a str {
        if let Some(list) = user_data.find_mailing_list(self.to_email.as_str()) {
            cli_print!("Found a mailing list : {} {}", list.name(), list.email());
            list.email()
        } else {
            // it's up to the sender to check if the address is valid
            self.to_email.as_str()
        }
    }

    fn select_range_interactively(&self, repo: &RepoData) -> Option<(String, String)> {
        cli_print!(
            "Press y for both the first and last commits of your \
            series are printed, any other key otherwise"
        );
        // acc = (first commit, last_commit, error)
        let mut first_last: (Option<String>, Option<String>, bool) = (None, None, false);

        let res = repo.open_git_repo()?.walk_from_head(&mut |commit| {
            cli_print!("Commit : {}", commit.hash());
            cli_print!("Summary: {}", commit.short_name());
            let mut k = String::new();
            if io::stdin().read_line(&mut k).is_err() {
                cli_print_error!("Failed to read the input");
                first_last.2 = true;
                return true;
            }
            cli_print!(); // new line
            if k.trim().eq_ignore_ascii_case(YES_KEY) {
                if first_last.1.is_none() {
                    first_last.1 = Some(commit.hash());
                    cli_print!("{} marked as last commit{}", commit.hash(), LINE_SEP);
                } else {
                    first_last.0 = Some(commit.hash());
                    cli_print!("{} marked as first commit{}", commit.hash(), LINE_SEP);
                    return false;
                }
            }
            true
        });
        match res {
            Ok(_) => {
                if first_last.2 {
                    return None;
                }
                if let (Some(first), Some(last)) = (first_last.0, first_last.1) {
                    return Some((first, last));
                }
                cli_print_error!("Interactive input failed, abort");
                None
            }
            Err(e) => {
                cli_print_error!("{}", e);
                None
            }
        }
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
            interactive: false,
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
        let to_email = self.get_to_email(user_data);
        let mut first_commit = self.first_commit.clone();
        let mut last_commit = self.last_commit.clone();
        if self.interactive {
            if let Some((f, l)) = self.select_range_interactively(repo) {
                (first_commit, last_commit) = (Some(f), Some(l))
            } else {
                return ControlFlow::Break(());
            }
        }
        let send_res = sender.send(
            series,
            to_email,
            &rtmp,
            first_commit.as_ref().unwrap(),
            last_commit.as_ref().unwrap(),
            self.cc.as_deref(),
        );

        match send_res {
            Ok(_) => {
                SeriesLog::send(series, to_email);
                ControlFlow::Continue(())
            }
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

        if !self.interactive && self.first_commit.is_none() {
            self.first_commit = Some(String::from(value));
            return Ok(());
        }

        if !self.interactive && self.last_commit.is_none() {
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
        match flag {
            CC_FLAG => {
                let value = value.trim();
                self.cc = Some(String::from(value));
                Ok(())
            }
            _ => Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::UnknownFlag,
                String::from(flag),
            )),
        }
    }

    fn add_flag(&mut self, flag: &str) -> Result<(), CommandBuilderError> {
        match flag {
            INTERACTIVE_FLAG => {
                self.interactive = true;
                Ok(())
            }
            _ => Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::UnknownFlag,
                String::from(flag),
            )),
        }
    }

    fn requires_value(&self, flag: &str) -> Result<bool, CommandBuilderError> {
        match flag {
            CC_FLAG => Ok(true),
            INTERACTIVE_FLAG => Ok(false),
            _ => Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::UnknownFlag,
                String::from(flag),
            )),
        }
    }

    fn name(&self) -> &str {
        SEND_SERIES
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(series_name), Some(to_email)) = (&self.series_name, &self.to_email) {
            let mut s = Box::new(SendSeries::new(
                series_name.as_str(),
                to_email.as_str(),
                self.cc.as_deref(),
                self.interactive,
            ));
            if !self.interactive {
                if let (Some(f), Some(l)) = (&self.first_commit, &self.last_commit) {
                    s.set_commit_range(f, l);
                } else {
                    return Err(CommandBuilderError::new(
                        super::CommandBuilderErrorCode::MissingValue,
                        String::from("Missing first or last commit sha1"),
                    ));
                }
            }
            Ok(s)
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing arguments"),
            ))
        }
    }
}
