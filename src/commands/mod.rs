mod echo_command;
mod exit_command;
mod type_command;
use std::{
    io::{stderr, stdin},
    path::{MAIN_SEPARATOR, Path},
    process::{Command, Stdio, exit},
};

pub const TYPE_COMMAND: &str = "type";
pub const EXIT_COMMAND: &str = "exit";
pub const ECHO_COMMAND: &str = "echo";

pub const COMMAND_LIST: [&str; 3] = [TYPE_COMMAND, EXIT_COMMAND, ECHO_COMMAND];

pub fn execute_command(argv: Vec<String>) {
    match argv[0].as_str() {
        EXIT_COMMAND => exit_command::execute(127),
        ECHO_COMMAND => echo_command::execute(argv),
        TYPE_COMMAND => type_command::execute(argv, Vec::from(COMMAND_LIST)),
        _ => try_in_path(argv),
    }
}

pub fn get_command() -> Vec<String> {
    let mut buffer = String::new();
    let command = match stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim(),
        Err(err) => {
            println!("{err}");
            exit(1);
        }
    };

    command
        .split(' ')
        .map(|str: &str| String::from(str))
        .collect()
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
        result = Command::new(&path)
            .current_dir(dir)
            .args(&argv[1..argv.len()])
            .status();
    } else {
        result = Command::new(&path).current_dir(dir).status();
    }

    if result.is_ok() {
        exit(result.unwrap().code().unwrap_or(0))
    } else {
        println!("{}", result.err().unwrap());
        exit(1)
    }
}

fn command_not_found(text: &str) -> () {
    println!("{text}: command not found")
}
