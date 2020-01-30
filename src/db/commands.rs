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
    InvalidCommandErr(String),
    SyntaxErr(String),
    Exit,
}

pub fn parse(command_string: String) -> Command {
    Command::Exit
}
