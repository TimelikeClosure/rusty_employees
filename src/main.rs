mod db;
mod io;
use db::QueryResponse::{Exit, Message, NoOp, Table};

fn main() {
    io::print_message(String::from(
        "\nWelcome to Departmental Employee Tracking System (TM)!\n",
    ));
    loop {
        io::print_message(String::from(
            "Enter query (Type \"Help\" for list of commands):",
        ));
        match db::query(io::get_query()) {
            NoOp => continue,
            Exit => break,
            Message(message) => io::print_message(message),
            Table(table) => io::print_table(table),
        }
    }
    io::print_message(String::from("\nThank you for using Departmental Employee Tracking System (TM) for you labor tracking needs!\n"));
}
