use std::ops::ControlFlow;

use log::debug;

use crate::{user_data::user_data::UserData, util::next_verbose_level};

use super::Command;

pub struct SetVerbose {}
impl SetVerbose {
    pub fn new() -> Self {
        Self {}
    }
}
impl Command for SetVerbose {
    fn exec(&self, _user_data: &mut UserData) -> ControlFlow<()> {
        let new_level = next_verbose_level(log::max_level());
        debug!("Set verbose to {new_level}");
        log::set_max_level(new_level);
        ControlFlow::Continue(())
    }
}
