mod echo_command;
mod exit_command;
mod type_command;
use std::{io::stdin, process::exit};

pub const TYPE_COMMAND: &str = "type";
pub const EXIT_COMMAND: &str = "exit";
pub const ECHO_COMMAND: &str = "echo";

pub const COMMAND_LIST: [&str; 3] = [TYPE_COMMAND, EXIT_COMMAND, ECHO_COMMAND];

pub fn execute_command(argv: Vec<String>) {
    match argv[0].as_str() {
        EXIT_COMMAND => exit_command::execute(127),
        ECHO_COMMAND => echo_command::execute(argv),
        TYPE_COMMAND => type_command::execute(argv, Vec::from(COMMAND_LIST)),
        _ => command_not_found(&argv[0]),
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

fn command_not_found(text: &str) -> () {
    println!("{text}: command not found")
}
