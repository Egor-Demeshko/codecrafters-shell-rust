use std::env;
use std::path::Path;

pub fn execute(argv: Vec<String>) -> () {
    if argv.len() <= 1 {
        println!("Missed path argument");
        return;
    }

    let validated_path = match validate_path(argv[1].as_str()) {
        Some(path) => path,
        None => {
            error_msg(argv[1].as_str());
            return;
        }
    };

    let path = Path::new(validated_path);
    if !path.exists() {
        error_msg(path.to_str().unwrap_or(""));
        return;
    }

    match env::set_current_dir(path) {
        Ok(result) => result,
        Err(e) => error_msg(e.to_string().as_str()),
    }
}

fn validate_path(path: &str) -> Option<&str> {
    if path.is_empty() {
        return None;
    } else {
        return Some(path);
    }
}

fn error_msg(path: &str) {
    println!("cd: {}: No such file or directory", path);
}
