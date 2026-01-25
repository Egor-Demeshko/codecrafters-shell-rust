use std::env;

use crate::domains::execute_command::ExecuteOptions;

pub fn execute(options: &ExecuteOptions) -> () {
    match env::current_dir() {
        Ok(path) => options.output(format!("{}\n", path.display()).as_str()),
        Err(e) => eprintln!("{}", e),
    }
}
