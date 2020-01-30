mod commands;

use commands::Command;

pub enum QueryResponse {
    Exit,
    NoOp,
    Message(String),
}

pub fn query(query_string: String) -> QueryResponse {
    // Steps to completed execution
    // 1. Tokenize & parse query string into command (or return err on missing command / invalid command syntax)
    // 2. Execute command
    // 3. Format response
    match commands::parse(query_string) {
        Command::EmptyCommand => QueryResponse::NoOp,
        Command::Exit => QueryResponse::Exit,
        Command::InvalidCommandErr(command) => QueryResponse::Message(
            format!("ERROR: Invalid command \"{command}\". Please check your spelling, or type \"Help\" for the list of available commands", command = command)
        ),
        Command::SyntaxErr(syntax_error_message) => QueryResponse::Message(
            format!("ERROR: Invalid command syntax: {}", syntax_error_message)
        ),
        Command::Help => QueryResponse::Message(
            commands::help()
        ),
    }
}
