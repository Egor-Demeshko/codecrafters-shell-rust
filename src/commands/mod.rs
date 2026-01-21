mod cd_command;
mod echo_command;
mod exit_command;
mod pwd_command;
mod type_command;

use std::{
    io::stdin,
    path::Path,
    process::{Command, exit},
};

pub const TYPE_COMMAND: &str = "type";
pub const EXIT_COMMAND: &str = "exit";
pub const ECHO_COMMAND: &str = "echo";
pub const PWD_COMMAND: &str = "pwd";
pub const CD_COMMAND: &str = "cd";

pub const COMMAND_LIST: [&str; 5] = [
    TYPE_COMMAND,
    EXIT_COMMAND,
    ECHO_COMMAND,
    PWD_COMMAND,
    CD_COMMAND,
];

pub fn execute_command(argv: Vec<String>) {
    match argv[0].as_str() {
        EXIT_COMMAND => exit_command::execute(127),
        ECHO_COMMAND => echo_command::execute(argv),
        TYPE_COMMAND => type_command::execute(argv, Vec::from(COMMAND_LIST)),
        PWD_COMMAND => pwd_command::execute(),
        CD_COMMAND => cd_command::execute(argv),
        _ => try_in_path(argv),
    }
}

pub fn get_command() -> Vec<String> {
    let mut buffer = String::new();
    let command = match stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().replace("''", ""),
        Err(err) => {
            println!("{err}");
            exit(1);
        }
    };

    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in command.chars() {
        match (ch, in_quotes) {
            ('\'', false) => in_quotes = true,
            ('\'', true) => in_quotes = false,
            (' ', false) if !current.is_empty() => {
                result.push(current);
                current = String::new();
            }
            (' ', false) => {}
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn try_in_path(argv: Vec<String>) -> () {
    let command_name = &argv[0];
    let path: String = match type_command::search_in_path(command_name.as_str()) {
        Some(path) => path,
        None => {
            command_not_found(command_name.as_str());
            return;
        }
    };

    let path = path.replace(command_name, "");
    let dir = Path::new(&path);

    let result: Result<std::process::ExitStatus, std::io::Error>;
    if argv.len() > 1 {
        result = Command::new(command_name)
            .current_dir(dir)
            .args(&argv[1..argv.len()])
            .status();
    } else {
        result = Command::new(command_name).current_dir(dir).status();
    }

    if result.is_ok() {
        return;
    } else {
        println!("{}", result.err().unwrap());
    }
}

fn command_not_found(text: &str) -> () {
    println!("{text}: command not found")
}
