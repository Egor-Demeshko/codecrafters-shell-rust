use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

fn main() {
    print!("$ ");
    stdout().flush().unwrap();
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => {
            println!("{}: command not found", buffer.trim());
        }
        Err(err) => {
            println!("{err}");
        }
    }
    exit(1);
}
