use crate::domains::execute_command::ExecuteOptions;
use std::{
    env,
    fs::{self},
    os::unix::fs::PermissionsExt,
    path::{MAIN_SEPARATOR, Path},
};

pub fn execute(options: &ExecuteOptions, command_list: Vec<&str>) -> () {
    let argv: &Vec<String> = options.get_argv();
    if argv[1].is_empty() {
        println!("provide command name to search for, ex. <type exit>");
        return;
    }
    let command_name = argv[1].as_str();

    match check_command_list(command_name, &command_list) {
        Some(command) => {
            options.output(format!("{command} is a shell builtin\n").as_str());
            return;
        }
        None => (),
    }

    let found_path = match search_in_path(command_name) {
        Some(path) => path,
        None => {
            println!("{command_name}: not found");
            return;
        }
    };

    options.output(format!("{} is {}\n", command_name, found_path.as_str()).as_str());
}

pub fn check_command_list<'a, 'b>(
    command_name: &'a str,
    command_list: &Vec<&'b str>,
) -> Option<&'b str> {
    let option = command_list.iter().find(|value| **value == command_name);
    if option.is_some() {
        Some(option.unwrap())
    } else {
        None
    }
}

pub fn search_in_path(command: &str) -> Option<String> {
    match env::var("PATH") {
        Ok(routes) => {
            let mut all_path: Vec<String> = vec![];
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
                all_path.push(text);
            }

            if all_path.is_empty() {
                return None;
            }

            for path in all_path.iter() {
                if path.starts_with(
                    format!(
                        "{MAIN_SEPARATOR}{}",
                        ["usr", "bin"].join(MAIN_SEPARATOR.to_string().as_str())
                    )
                    .as_str(),
                ) {
                    return Some(path.clone());
                }
            }

            Some(all_path[0].clone())
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
