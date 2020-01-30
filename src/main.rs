mod db;
mod io;
use db::QueryResponse::{Exit, Message};

fn main() {
    io::print_message(String::from(
        "\nWelcome to Departmental Employee Tracking System (TM)!\n",
    ));
    loop {
        match db::query(io::get_query()) {
            Exit => break,
            Message(message) => io::print_message(message),
        }
    }
    io::print_message(String::from("\nThank you for using Departmental Employee Tracking System (TM) for you labor tracking needs!\n"));
}
