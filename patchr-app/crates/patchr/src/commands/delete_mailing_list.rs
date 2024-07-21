use std::ops::ControlFlow;

use common::util::rust::result_to_control_flow;

use log::debug;

use crate::{cli_print, cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, DELETE_LIST};

pub struct DeleteMailingList {
    name: String,
}
pub struct DeleteMailingListBuilder {
    name: Option<String>,
}

impl DeleteMailingList {
    fn new(name: &str) -> Self {
        DeleteMailingList {
            name: String::from(name),
        }
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(DeleteMailingListBuilder::new())
    }
}

impl DeleteMailingListBuilder {
    fn new() -> Self {
        Self { name: None }
    }
}

impl Command for DeleteMailingList {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Deleting a list : {}", self.name);
        result_to_control_flow(user_data.delete_mailing_list(self.name.as_str()), |e| {
            cli_print_error!("{}", e);
            ()
        })?;
        cli_print!("List deleted");
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for DeleteMailingListBuilder {
    fn add_value(&mut self, value: &str) -> Result<(), CommandBuilderError> {
        if self.name.is_none() {
            self.name = Some(String::from(value));
            return Ok(());
        }

        Err(CommandBuilderError::unexpected_value(value))
    }

    fn name(&self) -> &str {
        DELETE_LIST
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        if let Some(name) = self.name.as_ref() {
            Ok(Box::new(DeleteMailingList::new(name.as_str())))
        } else {
            Err(CommandBuilderError::new(
                super::CommandBuilderErrorCode::MissingValue,
                String::from("Missing list name"),
            ))
        }
    }
}
