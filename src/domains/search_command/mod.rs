use crate::domains::search_command::search_in_path::search_in_path;
pub mod search_in_path;

pub fn gather_commands(build_in_commands: [&str; 5]) -> Vec<String> {
    let path_commands_res = search_in_path();
    let mut command_list: Vec<String> = Vec::from(build_in_commands)
        .iter()
        .map(|line| line.to_string())
        .collect();
    let mut path_commands = match path_commands_res {
        Ok(commands) => commands,
        Err(e) => {
            println!("Commands was not read from path with error: {}", e);
            vec![]
        }
    };
    command_list.append(&mut path_commands);
    command_list
}
