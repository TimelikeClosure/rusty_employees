pub enum Command {
    EmptyCommand,
    InvalidCommandErr(String),
    SyntaxErr(String),
    Exit,
    Help,
    ShowDepartments,
    // ListEmployees,
    // ListEmployeesByDepartment,
    ListEmployeesInDepartment(String),
    FormDepartment(String),
    // AssignEmployeeToDepartment(String, String),
    // TransferEmployeeBetweenDepartments(String, String, String),
    // PullEmployeeFromDepartment(String, String),
    // DissolveDepartment(String),
}

pub fn parse(command_string: String) -> Command {
    let mut tokens = command_string.split_whitespace();
    let command_prefix = tokens.next();
    match command_prefix {
        None => Command::EmptyCommand,
        Some(command_string) => match command_string.to_uppercase().as_str() {
            "EXIT" | "QUIT" | "LEAVE" | "BYE" => Command::Exit,
            "HELP" | "HALP" => Command::Help,
            "SHOW" => {
                let table = tokens.next();
                match table {
                    None => Command::SyntaxErr(String::from(
                        "\"Show\" command must specify a list name",
                    )),
                    Some(list_name) => match list_name.to_uppercase().as_str() {
                        "DEPARTMENTS" | "DEPT" | "DEPARTMENT" | "DEPTS" => match tokens.next() {
                            None => Command::ShowDepartments,
                            Some(extra_token) => Command::SyntaxErr(format!(
                                "Unexpected token \"{}\" after list name \"{}\"",
                                extra_token, list_name
                            )),
                        },
                        _ => Command::SyntaxErr(format!(
                            "Cannot show \"{}\": list does not exist",
                            list_name
                        )),
                    },
                }
            }
            "LIST" => match tokens.next() {
                None => {
                    Command::SyntaxErr(String::from("\"List\" command must specify a list name"))
                }
                Some(list_name) => match list_name.to_uppercase().as_str() {
                    "EMPLOYEES" | "EMPLOYEE" => match tokens.next() {
                        None => Command::InvalidCommandErr(String::from(command_string)),
                        Some(group_op) => match group_op.to_uppercase().as_str() {
                            "BY" => Command::InvalidCommandErr(String::from(command_string)),
                            "IN" => match tokens.next() {
                                None => Command::SyntaxErr(String::from(
                                    "Command \"List employees in\" must specify a department name",
                                )),
                                Some(department_name) => match tokens.next() {
                                    None => Command::ListEmployeesInDepartment(
                                        department_name.to_string(),
                                    ),
                                    Some(extra_token) => Command::SyntaxErr(format!(
                                        "Unexpected token \"{}\" after department name \"{}\"",
                                        extra_token, department_name
                                    )),
                                },
                            },
                            _ => Command::SyntaxErr(format!(
                                "Unexpected token \"{}\" after list name \"{}\"",
                                group_op, list_name,
                            )),
                        },
                    },
                    _ => Command::SyntaxErr(format!(
                        "Cannot list \"{}\": list does not exist",
                        list_name,
                    )),
                },
            },
            "FORM" => match tokens.next() {
                None => Command::SyntaxErr(String::from(
                    "\"Form\" command must specify a department to form",
                )),
                Some(department) => match tokens.next() {
                    Some(_) => Command::SyntaxErr(String::from(
                        "Due to company policy, department names can only be one word long",
                    )),
                    None => Command::FormDepartment(department.to_string()),
                },
            },
            // "DISSOLVE" => {},
            _ => Command::InvalidCommandErr(String::from(command_string)),
        },
    }
}

pub fn help() -> String {
    const HELP_MESSAGE: &str = ("\
        \nAvailable Operations:\
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
