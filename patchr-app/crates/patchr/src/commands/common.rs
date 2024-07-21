use std::{fs, process};

use log::debug;

use crate::{
    cli_print_error,
    user_data::{root_file::UserConfig, user_data::new_root_tmp_child_path},
};

pub fn edit_in_text_editor(user_config: &UserConfig, content: &str) -> Option<String> {
    debug!("Edit series");
    let Some(editor) = user_config.editor() else {
        cli_print_error!("Please define an editor");
        return None;
    };
    let editor = String::from(editor);
    let path = new_root_tmp_child_path().ok()?;
    if let Err(e) = fs::write(&path, content) {
        cli_print_error!("Cannot edit the content: {}", e.to_string());
        return None;
    };
    let editor_res = process::Command::new(editor).arg(&path).status();
    if let Err(e) = editor_res {
        cli_print_error!("Editor process returned an error: {}", e.to_string());
        let _ = fs::remove_file(&path); // at least we tried
        return None;
    }

    let read_res = fs::read_to_string(&path);
    let _ = fs::remove_file(&path);

    match read_res {
        Ok(new_content) => Some(new_content),
        Err(e) => {
            cli_print_error!("Failed to read the new value: {}", e.to_string());
            None
        }
    }
}

#[macro_export]
macro_rules! get_repo_or_fail {
    ($user_data:ident) => {
        if let Some(repo) = $user_data.repo() {
            repo
        } else {
            cli_print_error!("Not in a repo");
            return ControlFlow::Break(());            
        }
    };
}

#[macro_export]
macro_rules! get_repo_mut_or_fail {
    ($user_data:ident) => {
        if let Some(repo) = $user_data.repo_mut() {
            repo
        } else {
            cli_print_error!("Not in a repo");
            return ControlFlow::Break(());            
        }
    };
}

#[macro_export]
macro_rules! open_git_repo_or_fail {
    ($repo_data:ident) => {
        match $repo_data.open_git_repo() {
            Some(r) => r,
            None => {
                cli_print!("Repo cannot be opened");
                return ControlFlow::Break(());
            }
        }
    };
}