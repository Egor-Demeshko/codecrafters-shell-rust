use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

fn main() {
    loop {
        output("$ ");
        let mut buffer = String::new();
        let command = match stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim(),
            Err(err) => {
                println!("{err}");
                exit(1);
            }
        };

        match command {
            "exit" => exit(127),
            _ => println!("{}: command not found", command),
        }
    }
}

fn output(text: &str) {
    print!("{text}");
    match stdout().flush() {
        Ok(_) => return,
        Err(err) => {
            println!("{err}");
            exit(1);
        }
    }
}
