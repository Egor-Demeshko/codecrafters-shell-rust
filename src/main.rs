mod commands;
mod domains;

use commands::{execute_command, parse_command_entry};
use domains::execute_command::ExecuteOptions;

fn main() {
    loop {
        ExecuteOptions::standart_out("$ ");
        let execute_option = ExecuteOptions::new(parse_command_entry());
        execute_command(execute_option);
    }
}
