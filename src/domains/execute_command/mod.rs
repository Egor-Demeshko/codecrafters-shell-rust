use std::{
    fs::File,
    io::{Write, stdout},
    path::Path,
    process::exit,
};

pub const ARROW: &str = ">";
pub const UNIX_ARROW: &str = "1>";
pub const UNIX_ERROR_ARROW: &str = "2>";

pub enum OutputDestination {
    STANDART,
    FILE(String),
}

pub enum ErrorOutputDestination {
    STANDART,
    FILE(String),
}

pub struct ExecuteOptions {
    output_to: OutputDestination,
    error_to: ErrorOutputDestination,
    argv: Vec<String>,
    arguments: Vec<String>,
    command: String,
    pub exit_code: i32,
}

impl ExecuteOptions {
    pub fn new(argv: Vec<String>) -> Self {
        let (command, arguments, destination, error_destination) =
            ExecuteOptions::group_arguments(&argv);

        ExecuteOptions {
            output_to: destination,
            error_to: error_destination,
            argv,
            exit_code: 127,
            command,
            arguments,
        }
    }

    pub fn get_command_name(&self) -> &str {
        &self.command.as_str()
    }

    pub fn get_argv(&self) -> &Vec<String> {
        &self.argv
    }

    pub fn group_arguments(
        argv: &Vec<String>,
    ) -> (
        String,
        Vec<String>,
        OutputDestination,
        ErrorOutputDestination,
    ) {
        // skip first as it's command name
        let mut destination: OutputDestination = OutputDestination::STANDART;
        let mut error_destination: ErrorOutputDestination = ErrorOutputDestination::STANDART;
        let mut arguments = vec![];
        for i in 1..argv.len() {
            let mb_entry: Option<&String> = argv.get(i);
            if mb_entry.is_none() {
                destination = OutputDestination::STANDART;
            }
            let entry = mb_entry.unwrap();

            if entry == ARROW || entry == UNIX_ARROW {
                let empty_string = String::new();
                // if we received output operator we await next argument will be filename
                let file = argv.get(i + 1).unwrap_or(&empty_string);
                destination = OutputDestination::FILE(file.clone());
                break;
            }

            if entry == UNIX_ERROR_ARROW {
                let empty_string = String::new();
                let file = argv.get(i + 1).unwrap_or(&empty_string);
                error_destination = ErrorOutputDestination::FILE(file.clone())
            }

            arguments.push(entry.clone())
        }

        (
            argv.get(0).unwrap().clone(),
            arguments,
            destination,
            error_destination,
        )
    }

    pub fn output(&self, text: &str) -> () {
        match &self.output_to {
            OutputDestination::STANDART => ExecuteOptions::standart_out(text),
            OutputDestination::FILE(file_name) => self.file_out(text, file_name.as_str()),
        }
    }

    pub fn error_output(&self, text: &str) -> () {
        match &self.error_to {
            ErrorOutputDestination::STANDART => ExecuteOptions::standart_out(text),
            ErrorOutputDestination::FILE(file_name) => self.file_out(text, file_name.as_str()),
        }
    }

    pub fn standart_out(text: &str) -> () {
        print!("{text}");
        match stdout().flush() {
            Ok(_) => return,
            Err(err) => {
                println!("{err}");
                exit(1);
            }
        }
    }

    pub fn get_arguments(&self) -> &Vec<String> {
        &self.arguments
    }

    fn file_out(&self, text: &str, file_name: &str) -> () {
        let path = Path::new(file_name);
        let mut fds = match File::create(path) {
            Ok(ds) => ds,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        let result = fds.write(text.as_bytes());

        if result.is_err() {
            println!("{}", result.err().unwrap().to_string());
        }
    }
}
