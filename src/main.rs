mod db;
mod io;
use db::QueryResponse::{Exit, Message};

fn main() {
    println!("\nWelcome to Departmental Employee Tracking System (TM)!\n");
    loop {
        match db::query(io::get_query()) {
            Exit => break,
            Message(message) => io::print_message(message),
        }
    }
    println!("\nThank you for using Departmental Employee Tracking System (TM) for you labor tracking needs!\n");
}
