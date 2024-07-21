use std::{fmt::Display, ops::ControlFlow};

use log::debug;

use crate::{cli_print, cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, CommandBuilderErrorCode, CONFIGURE};

const DEL_FLAG: &str = "d";

#[derive(Debug, Clone, Copy)]
enum ConfigOption {
    Editor,
    SendCommand,
    FromEmail,
    SmtpServer,
    SmtpUser,
    SmtpPort,
    SmtpEncryption,
}

pub struct Configure {
    option: ConfigOption,
    value: Option<String>,
}

pub struct ConfigureBuilder {
    option: Option<ConfigOption>,
    value: Option<String>,
    delete: bool,
}

impl TryFrom<&str> for ConfigOption {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "editor" => Ok(ConfigOption::Editor),
            "sendcmd" => Ok(ConfigOption::SendCommand),
            "from" => Ok(ConfigOption::FromEmail),
            "smtpserver" => Ok(Self::SmtpServer),
            "smtpuser" => Ok(ConfigOption::SmtpUser),
            "smtpport" => Ok(ConfigOption::SmtpPort),
            "smtpenc" => Ok(ConfigOption::SmtpEncryption),
            _ => Err(()),
        }
    }
}

impl Display for ConfigOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ConfigOption::Editor => "editor",
            ConfigOption::SendCommand => "send command",
            ConfigOption::FromEmail => "source email",
            ConfigOption::SmtpServer => "smtp server",
            ConfigOption::SmtpUser => "smtp user",
            ConfigOption::SmtpPort => "smtp port",
            ConfigOption::SmtpEncryption => "smtp encryption",
        };
        f.write_str(name)
    }
}

impl Configure {
    fn new(option: ConfigOption, value: Option<&str>) -> Self {
        Configure {
            option,
            value: value.map(|s| { String::from(s) }),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(ConfigureBuilder::new())
    }
}

impl ConfigureBuilder {
    fn new() -> Self {
        Self {
            option: None,
            value: None,
            delete: false,
        }
    }
}

impl Command for Configure {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Configure");
        let config = user_data.config_mut();
        if let Some(value) = self.value.as_ref() {
            match self.option {
                ConfigOption::Editor => {
                    config.set_editor(value.as_str());
                }
                ConfigOption::FromEmail => {
                    config.set_from_email(value.as_str());
                }
                ConfigOption::SendCommand => {
                    config.set_send_command(self.value.as_ref().map(String::as_str));
                }
                ConfigOption::SmtpServer => {
                    config.set_smtp_server(self.value.as_ref().map(String::as_str));
                }
                ConfigOption::SmtpUser => {
                    config.set_smtp_user(self.value.as_ref().map(String::as_str));
                }
                ConfigOption::SmtpPort => {
                    if let Ok(target) = value.parse::<u16>() {
                        config.set_smtp_port(Some(target));
                    }
                }
                ConfigOption::SmtpEncryption => {
                    config.set_smtp_encryption(self.value.as_ref().map(String::as_str));
                }
            }
            cli_print!("New value for {}: '{}'", self.option, value.as_str());
        } else {
            match self.option {
                ConfigOption::SendCommand => {
                    config.set_send_command(None);
                }
                ConfigOption::SmtpServer => {
                    config.set_smtp_server(None);
                }
                ConfigOption::SmtpUser => {
                    config.set_smtp_user(None);
                }
                ConfigOption::SmtpPort => {
                    config.set_smtp_port(None);
                }
                ConfigOption::SmtpEncryption => {
                    config.set_smtp_encryption(None);
                }
                _ => {
                    cli_print_error!("{} cannot be unset", self.option);
                    return ControlFlow::Break(());
                }
            }
            cli_print!("{} unset", self.option);
        }
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for ConfigureBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.option.is_none() {
            if let Ok(option) = ConfigOption::try_from(value) {
                self.option = Some(option);
                return Ok(());
            }
            return Err(CommandBuilderError::unexpected_value(value));
        }

        if self.value.is_none() {
            self.value = Some(String::from(value));
            return Ok(());
        }

        Err(CommandBuilderError::unexpected_value(value))
    }

    fn add_flag(&mut self, flag: &str) -> Result<(), CommandBuilderError> {
        if flag == DEL_FLAG {
            self.delete = true;
            return Ok(());
        };

        Err(CommandBuilderError::new(
            CommandBuilderErrorCode::UnknownFlag,
            String::from(flag),
        ))
    }

    fn requires_value(&self, flag: &str) -> Result<bool, CommandBuilderError> {
        if flag == DEL_FLAG {
            Ok(false)
        } else {
            Err(CommandBuilderError::new(
                CommandBuilderErrorCode::UnknownFlag,
                String::from(flag),
            ))
        }
    }

    fn name(&self) -> &str {
        CONFIGURE
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        let Some(option) = self.option else {
            return Err(CommandBuilderError::new(
                CommandBuilderErrorCode::MissingValue,
                String::from("Missing option"),
            ));
        };

        if self.value.is_some() {
            if self.delete {
                return Err(CommandBuilderError::new(
                    CommandBuilderErrorCode::IncompatibleValues,
                    format!("Cannot provide a value whith -{}", DEL_FLAG),
                ));
            }
            return Ok(Box::new(Configure::new(option, self.value.as_ref().map(String::as_str))));
        };

        if self.delete {
            return Ok(Box::new(Configure::new(option, None)));
        }

        Err(CommandBuilderError::new(
            CommandBuilderErrorCode::MissingValue,
            String::from("Invalid arguments"),
        ))
    }
}
