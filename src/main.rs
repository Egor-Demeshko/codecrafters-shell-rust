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

        let argv: Vec<&str> = command.split(' ').collect();

        match argv[0] {
            "exit" => exit(127),
            "echo" => echo_command(argv),
            _ => println!("{}: command not found", argv[0]),
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

fn echo_command(argv: Vec<&str>) {
    let text = argv[1..argv.len()].join(" ");
    output(format!("{text}\n",).as_str());
}
