use std::ops::ControlFlow;

use common::util::rust::result_to_control_flow;

use log::debug;

use crate::{cli_print, cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, ADD_LIST};

pub struct AddMailingList {
    name: String,
    email: String,
}
pub struct AddMailingListBuilder {
    name: Option<String>,
    email: Option<String>,
}

impl AddMailingList {
    fn new(name: &str, email: &str) -> Self {
        AddMailingList {
            name: String::from(name),
            email: String::from(email),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(AddMailingListBuilder::new())
    }
}

impl AddMailingListBuilder {
    fn new() -> Self {
        Self {
            name: None,
            email: None,
        }
    }
}

impl Command for AddMailingList {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Adding a new list : {} {}", self.name, self.email);
        result_to_control_flow(
            user_data.add_mailing_list(self.name.as_str(), self.email.as_str()),
            |e| {
                cli_print_error!("{}", e);
                ()
            },
        )?;
        cli_print!("List added");
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for AddMailingListBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.name.is_none() {
            self.name = Some(String::from(value));
            return Ok(());
        }

        if self.email.is_none() {
            self.email = Some(String::from(value));
            return Ok(());
        }

        Err(CommandBuilderError::new(
            super::CommandBuilderErrorCode::UnexpectedValue,
            String::from(value),
        ))
    }

    fn name(&self) -> &str {
        ADD_LIST
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let (Some(name), Some(email)) = (self.name.as_ref(), self.email.as_ref()) {
            Ok(Box::new(AddMailingList::new(name.as_str(), email.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing arguments"),
            ))
        }
    }
}
