use std::io::{Write, stdin, stdout};

pub fn get_query() -> String {
    let mut input = String::new();
    loop {
        print!("> ");
        stdout().flush().expect("Error writing to stdout");
        match stdin().read_line(&mut input) {
            Err(_) => continue,
            Ok(_) => return input,
        }
    }
}

pub fn print_message(message: String) {
    println!("{}", message);
}
