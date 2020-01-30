pub enum Command {
    // Available Operations:
    // - "Help" - display available operations
    // - "Exit" - quits the program
    // - "Show departments" - list departments alphabetically
    // - "List employees" - list employees alphabetically
    // - "List employees by department" - list employees and their dept, grouped by dept. alphabetically, sorted alphabetically
    // - "List employees in {department}" - list employees in a dept, sorted alphabetically
    // - "Form {department}" - create new department
    // - "Assign {employee} to {department}" - create new employee under department
    // - "Transfer {employee} from {department} to {department}" - move employee from first department to second
    // - "Pull {employee} from {department}" - remove employee from department
    // - "Dissolve {department}" - remove department and all employees in it
    EmptyCommand,
    InvalidCommandErr(String),
    SyntaxErr(String),
    Exit,
    Help,
}

pub fn parse(command_string: String) -> Command {
    let mut tokens = command_string.split_whitespace();
    let command_prefix = tokens.next();
    match command_prefix {
        None => Command::EmptyCommand,
        Some(command_string) => match command_string.to_uppercase().as_str() {
            "EXIT" => Command::Exit,
            "HELP" => Command::Help,
            _ => Command::InvalidCommandErr(String::from(command_string)),
        },
    }
}

pub fn help() -> String {
    const HELP_MESSAGE: &str = ("\
        Available Operations:\
        \n- \"Help\" - display available operations (this help message)\
        \n- \"Exit\" - quits the program\
        \n- \"Show departments\" - list departments alphabetically\
        \n- \"List employees\" - list employees alphabetically\
        \n- \"List employees by department\" - list employees and their dept, grouped by dept. alphabetically, sorted alphabetically\
        \n- \"List employees in {department}\" - list employees in a dept, sorted alphabetically\
        \n- \"Form {department}\" - create new department\
        \n- \"Assign {employee} to {department}\" - create new employee under department\
        \n- \"Transfer {employee} from {department} to {department}\" - move employee from first department to second\
        \n- \"Pull {employee} from {department}\" - remove employee from department\
        \n- \"Dissolve {department}\" - remove department and all employees in it\
    \n");
    String::from(HELP_MESSAGE)
}
