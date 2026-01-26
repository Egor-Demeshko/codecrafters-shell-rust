use std::env;
use std::path::Path;

use crate::domains::execute_command::ExecuteOptions;

pub fn execute(options: &ExecuteOptions) -> () {
    let argv = options.get_argv();
    if argv.len() <= 1 {
        options.error_output(format!("{}\n", "Missed path argument").as_str());
        return;
    }

    let mut validated_path = match validate_path(argv[1].as_str()) {
        Some(path) => path,
        None => {
            error_msg(argv[1].as_str(), &options);
            return;
        }
    };

    if validated_path == "~" {
        let home: String = env::var("HOME").unwrap_or(String::new());
        if home.is_empty() {
            error_msg("~", &options);
            return;
        }

        validated_path = home.leak()
    }

    let path = Path::new(validated_path);
    if !path.exists() {
        error_msg(path.to_str().unwrap_or(""), &options);
        return;
    }

    match env::set_current_dir(path) {
        Ok(result) => result,
        Err(e) => error_msg(e.to_string().as_str(), &options),
    }
}

fn validate_path(path: &str) -> Option<&str> {
    if path.is_empty() {
        return None;
    } else {
        return Some(path);
    }
}

fn error_msg(path: &str, options: &ExecuteOptions) {
    options.error_output(format!("cd: {}: No such file or directory\n", path).as_str());
}
