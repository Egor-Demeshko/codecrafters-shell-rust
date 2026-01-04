use std::{
    io::{Write, stdout},
    process::exit,
};

pub fn output(text: &str) -> () {
    print!("{text}");
    match stdout().flush() {
        Ok(_) => return,
        Err(err) => {
            println!("{err}");
            exit(1);
        }
    }
}
