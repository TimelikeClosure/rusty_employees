use std::collections::HashMap;

mod commands;
use commands::Command;
mod store;
use store::Store;

pub struct Table {
    pub title: String,
    pub headers: Vec<String>,
    pub data: Vec<HashMap<String, String>>,
}

pub enum QueryResponse {
    Exit,
    NoOp,
    Message(String),
    Table(Table),
}

pub struct Database {
    store: Store,
}

impl Database {
    pub fn new() -> Database {
        Database {
            store: Store::new(),
        }
    }

    pub fn query(&mut self, query_string: String) -> QueryResponse {
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
            Command::ShowDepartments => {
                let departments = self.store.departments.list();
                const COLUMN_NAME: &str = "Department";
                QueryResponse::Table(
                    Table {
                        title: String::from("Showing all Departments"),
                        headers: vec![COLUMN_NAME.to_string()],
                        data: departments.iter().map(|dept_name| {
                            let mut row = HashMap::new();
                            row.insert(COLUMN_NAME.to_string(), dept_name.to_owned());
                            row
                        }).fold(Vec::new(), |mut rows, row| {
                            rows.push(row);
                            rows
                        }),
                    }
                )
            },
        }
    }
}