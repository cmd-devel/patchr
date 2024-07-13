use std::fs;
use std::ops::ControlFlow;
use std::path::Path;

use log::debug;

use crate::cli_print;
use crate::user_data::user_data::root_tmp_dir_path;
use crate::{cli_print_error, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, ADD_REVISION};

pub struct CleanTmp;

pub struct CleanTmpBuilder;

impl CleanTmp {
    fn new() -> Self {
        Self {}
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(CleanTmpBuilder::new())
    }

    fn list_tmp(dir: &Path) -> ControlFlow<()> {
        let it = match fs::read_dir(dir) {
            Ok(it) => it,
            Err(err) => {
                cli_print_error!("Error: {}", err);
                return ControlFlow::Break(());
            }
        };
        it.for_each(|elt| match elt {
            Ok(entry) => cli_print!("{}", entry.file_name().to_string_lossy()),
            Err(err) => cli_print_error!("Error: {}", err),
        });
        ControlFlow::Continue(())
    }
}

impl CleanTmpBuilder {
    fn new() -> Self {
        Self {}
    }
}

impl Command for CleanTmp {
    fn exec(&self, _user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Clean tmp");
        match root_tmp_dir_path() {
            Ok(dir) => {
                Self::list_tmp(dir.as_path())?;
                match fs::remove_dir_all(dir) {
                    Ok(()) => {
                        cli_print!("Done");
                        ControlFlow::Continue(())
                    }
                    Err(e) => {
                        cli_print_error!("{}", e);
                        ControlFlow::Break(())
                    }
                }
            }
            Err(e) => {
                cli_print_error!("{}", e);
                ControlFlow::Break(())
            }
        }
    }
}

impl CommandBuilder for CleanTmpBuilder {
    fn name(&self) -> &str {
        ADD_REVISION
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        Ok(Box::new(CleanTmp::new()))
    }
}
