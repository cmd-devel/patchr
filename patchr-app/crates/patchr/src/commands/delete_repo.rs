use std::ops::ControlFlow;

use common::util::rust::result_to_control_flow;
use log::debug;

use crate::{cli_print, cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, DELETE_REPO};

pub struct DeleteRepo;

pub struct DeleteRepoBuilder;

impl DeleteRepo {
    fn new() -> Self {
        DeleteRepo {}
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(DeleteRepoBuilder::new())
    }
}

impl DeleteRepoBuilder {
    fn new() -> Self {
        Self {}
    }
}

impl Command for DeleteRepo {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Delete repo");
        result_to_control_flow(user_data.delete_repo(), |e| {
            cli_print_error!("{}", e);
            ()
        })?;
        cli_print!("Repo deleted");
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for DeleteRepoBuilder {
    fn name(&self) -> &str {
        DELETE_REPO
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        Ok(Box::new(DeleteRepo::new()))
    }
}
