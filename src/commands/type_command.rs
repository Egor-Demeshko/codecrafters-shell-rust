use crate::helpers::output;
use std::{
    env,
    fs::{self},
    os::unix::fs::PermissionsExt,
};

pub fn execute(argv: Vec<String>, command_list: Vec<&str>) -> () {
    if argv[1].is_empty() {
        println!("provide command name to search for, ex. <type exit>");
        return;
    }
    let command_name = argv[1].as_str();

    match command_list.iter().find(|value| **value == command_name) {
        Some(command) => {
            output(format!("{command} is a shell builtin\n").as_str());
            return;
        }
        None => (),
    }

    match search_in_path(command_name) {
        Some(path) => {
            output(path.as_str());
            return;
        }
        None => (),
    }

    println!("{command_name}: not found");
}

fn search_in_path(command: &str) -> Option<String> {
    match env::var("PATH") {
        Ok(routes) => {
            for route in env::split_paths(&routes) {
                let dir = route.as_path();
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let file_name = entry.file_name();
                        if file_name.to_string_lossy().contains(command) {
                            let file = fs::metadata(entry.path()).unwrap();
                            let permissions = file.permissions();
                            if permissions.mode() & 0o111 == 0 {
                                continue;
                            }
                            return Some(format!(
                                "{} is {}/{}\n",
                                command,
                                dir.to_str().unwrap(),
                                command
                            ));
                        }
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}
