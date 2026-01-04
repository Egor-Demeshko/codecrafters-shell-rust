use std::{
    default,
    io::{Write, stdin, stdout},
    process::exit,
};

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        let command = match stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim(),
            Err(err) => {
                println!("{err}");
                exit(1);
            }
        };

        match command {
            "exit" => exit(1),
            _ => println!("{}: command not found", command),
        }
    }
}
