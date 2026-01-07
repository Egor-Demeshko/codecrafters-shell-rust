use crate::helpers::output;
use std::{
    env,
    fs::{self},
    os::unix::fs::PermissionsExt,
    path::Path,
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
            let mut shortest_path = String::new();
            for route in env::split_paths(&routes) {
                if route.as_os_str().is_empty() {
                    continue;
                }
                let text = match search_path(route.as_path(), command) {
                    Some(text) => text,
                    None => continue,
                };
                if text.is_empty() {
                    continue;
                }
                if shortest_path.is_empty() || shortest_path.len() > text.len() {
                    shortest_path = text;
                }
            }

            if shortest_path.is_empty() {
                return None;
            }
            Some(format!("{} is {}\n", command, shortest_path))
        }
        Err(_) => None,
    }
}

fn search_path(dir: &Path, command: &str) -> Option<String> {
    let address = dir.join(command);
    let full_address = address.as_path();
    if !full_address.is_file() {
        return None;
    }

    if is_executable(full_address) {
        return Some(format!(
            "{}",
            full_address.as_os_str().to_str().unwrap_or("")
        ));
    }
    None
}

fn is_executable(path: &Path) -> bool {
    let metadata_res = fs::metadata(path);
    if metadata_res.is_err() {
        return false;
    }

    let metadata = metadata_res.unwrap();
    if metadata.permissions().mode() & 0o111 != 0 {
        return true;
    }
    return false;
}
