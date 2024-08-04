use std::ops::ControlFlow;

use log::debug;

use crate::{cli_print, cli_print_error, commands::common::edit_in_text_editor, user_data::user_data::UserData};

use super::{Command, CommandBuilder, CommandBuilderError, EDIT_CV_SKEL};

pub struct EditCVSkel;
pub struct EditCVSkelBuilder;

impl EditCVSkel {
    fn new() -> Self {
        EditCVSkel {}
    }

    pub fn builder() -> Box<dyn CommandBuilder> {
        Box::new(EditCVSkelBuilder::new())
    }
}

impl EditCVSkelBuilder {
    fn new() -> Self {
        Self {}
    }
}

impl Command for EditCVSkel {
    fn exec(&self, user_data: &mut UserData) -> ControlFlow<()> {
        debug!("Editing CV skel");

        let user_config = user_data.config_mut();
        let current_skel = user_config.cv_skel().unwrap_or("");

        let Some(new_content) = edit_in_text_editor(user_config, current_skel) else {
            cli_print_error!("Update failed, abort");
            return ControlFlow::Break(());
        };

        let new_content = match new_content.as_str().trim() {
            "" => None,
            s => Some(s),
        };
        user_config.set_cv_skel(new_content);
        cli_print!("CV skel edited");
        ControlFlow::Continue(())
    }
}

impl CommandBuilder for EditCVSkelBuilder {
    fn name(&self) -> &str {
        EDIT_CV_SKEL
    }

    fn build(&self) -> Result<Box<dyn Command>, CommandBuilderError> {
        Ok(Box::new(EditCVSkel::new()))
    }
}
