use std::ops::ControlFlow;

use common::constants::PROJECT_VERSION;

use crate::{cli_print, user_data::user_data::UserData};

use super::Command;

pub struct Help {}

impl Help {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Help {
    fn exec(&self, _user_data: &mut UserData) -> ControlFlow<()> {
        cli_print!(
            r##"patchr {}

patchr comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it under
certain conditions. See the GNU General Public Licence for details.
    
patchr is a git patch series management program.

Repo:
    - Register a new repo
        patchr register [name]

    - List repos
        patchr repos

    - Delete a repo
        patchr delrepo [name]
Series:
    The following commands can only be called from a registered repo

    - List series
        patchr list

    - Show a series
        patchr show [series]

    - Create a new series
        patchr create [name] [title]

    - Edit a series
        patchr edit [target] [series]
            target: 'cv', 'title', 'name' or 'short'
            series: series name

    - Add a revision
        patchr addrev [series]

    - Delete a revision
        patchr delrev [series] [rev]
            rev: number of the revision to edit

    - Edit a revision
        patchr editrev [series] [rev]
            rev: number of the revision to edit

    - Send a series
        patchr send [series] [c1] [c2] [to]
            c1: initial commit
            c2: last commit
            to: target mailing list

User configuration:
    - Edit global configuration
        patchr edit [target] [value]
            target: editor, sendcmd, from, smtpserver, smtpuser
            value: new value

Global flags:
    -v verbose (increate verbosity)
    -h print this help message
        "##,
            PROJECT_VERSION
        );
        ControlFlow::Break(()) // ignore the other commands
    }
}
