use crate::helpers::output;

pub fn execute(
    argv: Vec<String>,
    command_list: Vec<&str>,
    command_not_found: fn(text: &str) -> (),
) -> () {
    if argv[1].is_empty() {
        println!("provide command name to search for, ex. <type exit>");
        return;
    }
    let command_name = argv[1].as_str();
    match command_list.iter().find(|value| **value == command_name) {
        Some(command) => output(format!("{command} is a shell builtin\n").as_str()),
        None => command_not_found(command_name),
    }
}
