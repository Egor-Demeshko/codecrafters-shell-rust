mod commands;
mod data_structures;
mod domains;

use commands::{execute_command, parse_command_entry};
use domains::execute_command::ExecuteOptions;
use rustyline::{Editor, history::DefaultHistory};
use std::process::exit;

use crate::{commands::COMMAND_LIST, domains::hinter::ShellHinter};

fn main() {
    let mut rl: Editor<ShellHinter, DefaultHistory> = Editor::new().unwrap();
    let mut helper = ShellHinter::new();
    helper.set_commands(Vec::from(COMMAND_LIST));
    rl.set_helper(Some(helper));

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                let execute_option = ExecuteOptions::new(parse_command_entry(line));
                execute_command(execute_option);
            }
            Err(err) => {
                println!("{err}");
                exit(1);
            }
        };
    }
}
