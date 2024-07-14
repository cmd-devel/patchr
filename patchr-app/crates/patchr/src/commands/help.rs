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
        patchr register <name>

    - List repos
        patchr repos

    - Delete the current repo
        patchr delrepo

Series:
    The following commands can only be called from a registered repo

    - List series
        patchr list

    - Show a series
        patchr show [-v] <series>
            -v verbose

    - Create a new series
        patchr create <name> <title>

    - Delete a series
        patchr delete <name>

    - Edit a series
        patchr edit <target> <series>
            target: 'cv', 'cc' (see send command for format), 'title', 'name' or 'short'
            series: series name

    - Add a revision
        patchr addrev <series>

    - Delete a revision
        patchr delrev <series> <rev>
            rev: number of the revision to edit

    - Edit a revision
        patchr editrev <series> <rev>
            rev: number of the revision to edit

    - Send a series
        patchr send <series> <c1> <c2> <to> [-c email1,...]
            c1: initial commit
            c2: last commit
            to: target mailing list or mailing list name
            -c: allows to add addresses to the CC field (separated by commas)

User configuration:
    - Edit global configuration
        patchr config [-d] <target> [value]
            target: editor, sendcmd, from, smtpserver, smtpuser, smtpport, smtpenc
            value: new value (not compatible with -d)
            -d: delete the current value

    - Register a mailing list
        patchr addlist <listname> <email address>

    - Delete a mailing list
        patchr dellist <list name>

Patchr operations:
    - Delete temporary files
        patchr cleantmp

Global flags:
    -v verbose (increase verbosity, can be used multiple times)

    -h print this help message
        "##,
            PROJECT_VERSION
        );
        ControlFlow::Break(()) // ignore the other commands
    }
}
