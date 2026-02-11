mod commands;
mod data_structures;
mod domains;

use commands::{execute_command, parse_command_entry};
use domains::execute_command::ExecuteOptions;
use rustyline::{CompletionType, Config, Editor, history::DefaultHistory};
use std::process::exit;

use crate::{
    commands::COMMAND_LIST,
    domains::{hinter::ShellHinter, search_command::gather_commands},
};

fn main() {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .completion_show_all_if_ambiguous(false)
        .build();
    let mut rl: Editor<ShellHinter, DefaultHistory> = Editor::with_config(config).unwrap();
    let mut helper = ShellHinter::new();
    let command_list: Vec<String> = gather_commands(COMMAND_LIST);
    helper.set_commands(command_list);
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
