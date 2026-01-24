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

struct ParseCommand {
    current: String,
    active_quote: String,
    next_literal: bool,
    next_literal_filter: Vec<char>,
}

impl ParseCommand {
    fn new() -> Self {
        ParseCommand {
            current: String::new(),
            active_quote: String::new(),
            next_literal: false,
            next_literal_filter: vec![],
        }
    }

    pub fn get_current(&self) -> &String {
        &self.current
    }

    pub fn push_to_current(&mut self, ch: char) -> () {
        self.current.push(ch);
    }

    pub fn get_current_mut(&mut self) -> String {
        let current = self.current.clone();
        self.current = String::new();
        current
    }

    pub fn get_qoute(&self) -> &str {
        &self.active_quote
    }

    pub fn get_next_literal(&self) -> bool {
        self.next_literal
    }

    pub fn reset_active_quoute(&mut self) -> () {
        self.active_quote.truncate(0);
    }

    pub fn set_active_qoute(&mut self, ch: char) -> () {
        self.reset_active_quoute();
        self.active_quote.push(ch);
    }

    pub fn set_next_literal(&mut self, value: bool) -> &Self {
        self.next_literal_filter.clear();
        self.next_literal = value;
        self
    }

    pub fn is_next_literal(&self, ch: char) -> bool {
        if self.next_literal && !self.next_literal_filter.is_empty() {
            return match self.next_literal_filter.iter().find(|el: &&char| -> bool {
                if **el == ch { true } else { false }
            }) {
                Some(_) => true,
                None => false,
            };
        }
        self.next_literal
    }

    pub fn set_next_literal_filter(&mut self, filter: Vec<char>) -> () {
        self.next_literal_filter = filter;
    }
}

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
        Ok(_) => buffer.trim().replace("''", "").replace("\"\"", ""),
        Err(err) => {
            println!("{err}");
            exit(1);
        }
    };

    let mut result = Vec::new();
    let mut parse_command = ParseCommand::new();

    for ch in command.chars() {
        let active_qoute = parse_command.get_qoute();
        if active_qoute.is_empty() {
            when_active_quote_empty(ch, &mut parse_command, &mut result);
        } else if active_qoute == "\"" {
            char_in_double_quoutes(ch, &mut parse_command);
        } else if active_qoute == "\'" {
            char_in_single_quoute(ch, &mut parse_command);
        } else {
            parse_command.push_to_current(ch)
        }
    }

    if !parse_command.get_current().is_empty() {
        result.push(parse_command.get_current_mut());
    }

    result
}

fn when_active_quote_empty(ch: char, options: &mut ParseCommand, result: &mut Vec<String>) -> () {
    if options.is_next_literal(ch) {
        options.push_to_current(ch);
        options.set_next_literal(false);
    } else if ch == '\\' {
        options.set_next_literal(true);
    } else if ch == '\'' || ch == '\"' {
        options.set_active_qoute(ch);
    } else if ch == ' ' || ch.to_string() == "  " {
        if !options.get_current().is_empty() {
            result.push(options.get_current_mut());
        }
    } else {
        options.push_to_current(ch)
    }
}

fn char_in_single_quoute(ch: char, options: &mut ParseCommand) -> () {
    match ch {
        '\'' => options.reset_active_quoute(),
        _ => options.push_to_current(ch),
    }
}
// "exe with \'single quotes\'"
fn char_in_double_quoutes(ch: char, options: &mut ParseCommand) -> () {
    match (ch, options.get_next_literal()) {
        ('\\', false) => {
            options.set_next_literal(true);
            options.set_next_literal_filter(vec!['\"', '\\', '$', '\'', '\n']);
        }
        ('\"', false) => {
            options.reset_active_quoute();
            options.set_next_literal(false);
        }
        ('\\', true) => {
            options.push_to_current(ch);
            options.set_next_literal(false);
        }
        ('\"', true) => {
            options.push_to_current(ch);
            options.set_next_literal(false);
        }
        ('\'', true) => {
            options.set_next_literal(false);
        }
        _ => {
            options.push_to_current(ch);
            options.set_next_literal(false);
        }
    }
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
