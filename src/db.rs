pub enum QueryResponse {
    Exit,
    Message(String),
}

pub fn query(query_string: String) -> QueryResponse {
    // Available Operations:
    // - "Help" - display available operations
    // - "Exit" - quits the program
    // - "List departments" - list departments alphabetically
    // - "List employees" - list employees alphabetically
    // - "List employees by department" - list employees and their dept, grouped by dept. alphabetically, sorted alphabetically
    // - "List employees in {department}" - list employees in a dept, sorted alphabetically
    // - "Commission {department}" - create new department
    // - "Add {employee} to {department}" - create new employee under department
    // - "Move {employee} from {department} to {department}" - move employee from first department to second
    // - "Pull {employee} from {department}" - remove employee from department
    // - "Dissolve {department}" - remove department and all employees in it
    QueryResponse::Exit
}
