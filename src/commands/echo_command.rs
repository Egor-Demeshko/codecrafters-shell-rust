use crate::domains::execute_command::ExecuteOptions;

pub fn execute(options: &ExecuteOptions) -> () {
    let arguments = options.get_arguments();
    let text = arguments[0..arguments.len()].join(" ");
    options.output(format!("{text}\n",).as_str());
}
