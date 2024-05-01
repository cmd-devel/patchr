use std::{env, process::exit};

use log::{debug, trace};
use parser::parse_command_line;
use util::init_logger;

use crate::{error_code::ErrorCode, user_data::user_data::UserData};

mod cli_output;
mod commands;
mod environment;
mod error_code;
mod parser;
mod user_data;
mod util;

fn init_app() {
    init_logger();
    read_environment();
}

fn read_environment() {
    if env::var(environment::PATCHR_DBG).is_ok() {
        log::set_max_level(log::LevelFilter::max());
    }
}

fn load_user_data_or_die() -> UserData {
    match UserData::load() {
        Ok(user_data) => user_data,
        Err(e) => {
            cli_print_error!("Failed to load user data");
            debug!("{}", e.to_string());
            exit(ErrorCode::CannotReadUserData.code());
        }
    }
}

fn save_user_data_or_die(user_data: &UserData) {
    if let Err(e) = user_data
        .save_root_file()
        .and_then(|_| user_data.save_repo())
    {
        cli_print_error!("Failed to write data to disk {}", e);
        exit(ErrorCode::CannotWriteUserData.code());
    }
}

fn main() {
    init_app();
    let args: Vec<String> = env::args().collect();
    trace!("arguments : {}", args.join(" "));
    let Some(commands) = parse_command_line(args) else {
        exit(ErrorCode::ParsingError.code());
    };

    let mut user_data = load_user_data_or_die();
    let result = commands.iter().try_for_each(|c| c.exec(&mut user_data));

    if result.is_break() {
        exit(ErrorCode::CommandError.code());
    }

    save_user_data_or_die(&user_data);
}
