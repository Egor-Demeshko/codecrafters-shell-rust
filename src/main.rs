mod commands;
mod helpers;

use commands::{execute_command, get_command};
use helpers::output;

fn main() {
    loop {
        output("$ ");
        execute_command(get_command());
    }
}
