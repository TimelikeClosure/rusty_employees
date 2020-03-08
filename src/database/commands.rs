#[derive(Debug, PartialEq)]
pub enum Command {
    EmptyCommand,
    InvalidCommandErr(String),
    SyntaxErr(String),
    Exit,
    Help,
    ShowDepartments,
    ListEmployees,
    ListEmployeesByDepartment,
    ListEmployeesInDepartment(String),
    FormDepartment(String),
    AssignEmployeeToDepartment(String, String),
    TransferEmployeeBetweenDepartments(String, String, String),
    PullEmployeeFromDepartment(String, String),
    DissolveDepartment(String),
}

pub fn parse(command_string: String) -> Command {
    let mut tokens = command_string.split_whitespace();
    let command_prefix = tokens.next();
    match command_prefix {
        None => Command::EmptyCommand,
        Some(command_string) => match command_string.to_uppercase().as_str() {
            "EXIT" | "QUIT" | "LEAVE" | "BYE" => Command::Exit,
            "HELP" | "HALP" => Command::Help,
            "SHOW" => parse_show(tokens),
            "LIST" => match tokens.next() {
                None => {
                    Command::SyntaxErr(String::from("\"List\" command must specify a list name"))
                }
                Some(list_name) => match list_name.to_uppercase().as_str() {
                    "EMPLOYEES" | "EMPLOYEE" => match tokens.next() {
                        None => Command::ListEmployees,
                        Some(group_op) => match group_op.to_uppercase().as_str() {
                            "BY" => match tokens.next() {
                                None => Command::SyntaxErr(String::from(
                                    "\"List employees by\" must specify a group by field",
                                )),
                                Some(group_list) => match group_list.to_uppercase().as_str() {
                                    "DEPARTMENT" => match tokens.next() {
                                        None => Command::ListEmployeesByDepartment,
                                        Some(extra_token) => Command::SyntaxErr(format!(
                                            "Unexpected token \"{}\" after group by field \"{}\"",
                                            extra_token, group_list
                                        )),
                                    },
                                    _ => Command::SyntaxErr(format!(
                                        "\"{}\" is not a field employees can by grouped by",
                                        group_list
                                    )),
                                },
                            },
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
            "ASSIGN" => parse_assign(tokens),
            "TRANSFER" => parse_transfer(tokens),
            "PULL" => parse_pull(tokens),
            "FORM" => parse_form(tokens),
            "DISSOLVE" => parse_dissolve(tokens),
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

fn parse_assign<'a, T: DoubleEndedIterator<Item = &'a str>>(mut tokens: T) -> Command {
    const ASSIGN_SYNTAX_ERR: &str =
        "\"Assign\" command must specify an employee to assign and a department to assign to";
    match tokens.next_back() {
        None => Command::SyntaxErr(String::from(ASSIGN_SYNTAX_ERR)),
        Some(department) => match tokens.next_back() {
            None => Command::SyntaxErr(String::from(ASSIGN_SYNTAX_ERR)),
            Some(group_op) => match group_op.to_uppercase().as_str() {
                "TO" => match tokens.next() {
                    None => Command::SyntaxErr(String::from(ASSIGN_SYNTAX_ERR)),
                    Some(employee_first_name) => {
                        let mut employee = String::from(employee_first_name);
                        tokens.for_each(|token| {
                            employee.push(' ');
                            employee.push_str(token);
                        });
                        Command::AssignEmployeeToDepartment(employee, department.to_string())
                    }
                },
                _ => Command::SyntaxErr(String::from(ASSIGN_SYNTAX_ERR)),
            },
        },
    }
}

fn parse_dissolve<'a, T: Iterator<Item = &'a str>>(mut tokens: T) -> Command {
    match tokens.next() {
        None => Command::SyntaxErr(String::from(
            "\"Dissolve\" command must specify a department to dissolve",
        )),
        Some(department) => match tokens.next() {
            Some(_) => Command::SyntaxErr(String::from(
                "Due to company policy, department names can only be one word long",
            )),
            None => Command::DissolveDepartment(department.to_string()),
        },
    }
}

fn parse_form<'a, T: Iterator<Item = &'a str>>(mut tokens: T) -> Command {
    match tokens.next() {
        None => Command::SyntaxErr(String::from(
            "\"Form\" command must specify a department to form",
        )),
        Some(department) => match tokens.next() {
            Some(_) => Command::SyntaxErr(String::from(
                "Due to company policy, department names can only be one word long",
            )),
            None => Command::FormDepartment(department.to_string()),
        },
    }
}

fn parse_pull<'a, T: DoubleEndedIterator<Item = &'a str>>(mut tokens: T) -> Command {
    const PULL_SYNTAX_ERR: &str =
        "\"Pull\" command must specify an employee to pull and a department to pull from";
    match tokens.next_back() {
        None => Command::SyntaxErr(String::from(PULL_SYNTAX_ERR)),
        Some(department) => match tokens.next_back() {
            None => Command::SyntaxErr(String::from(PULL_SYNTAX_ERR)),
            Some(group_op) => match group_op.to_uppercase().as_str() {
                "FROM" => match tokens.next() {
                    None => Command::SyntaxErr(String::from(PULL_SYNTAX_ERR)),
                    Some(employee_first_name) => {
                        let mut employee = String::from(employee_first_name);
                        tokens.for_each(|token| {
                            employee.push(' ');
                            employee.push_str(token);
                        });
                        Command::PullEmployeeFromDepartment(employee, department.to_string())
                    }
                },
                _ => Command::SyntaxErr(String::from(PULL_SYNTAX_ERR)),
            },
        },
    }
}

fn parse_show<'a, T: Iterator<Item = &'a str>>(mut tokens: T) -> Command {
    let table = tokens.next();
    match table {
        None => Command::SyntaxErr(String::from("\"Show\" command must specify a list name")),
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

fn parse_transfer<'a, T: DoubleEndedIterator<Item = &'a str>>(mut tokens: T) -> Command {
    const TRANSFER_SYNTAX_ERR: &str = "\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to";
    match tokens.next_back() {
        None => Command::SyntaxErr(String::from(TRANSFER_SYNTAX_ERR)),
        Some(to_department) => match tokens.next_back() {
            None => Command::SyntaxErr(String::from(TRANSFER_SYNTAX_ERR)),
            Some(to_op) => match to_op.to_uppercase().as_str() {
                "TO" => match tokens.next_back() {
                    None => Command::SyntaxErr(String::from(TRANSFER_SYNTAX_ERR)),
                    Some(from_department) => match tokens.next_back() {
                        None => Command::SyntaxErr(String::from(TRANSFER_SYNTAX_ERR)),
                        Some(from_op) => match from_op.to_uppercase().as_str() {
                            "FROM" => match tokens.next() {
                                None => Command::SyntaxErr(String::from(TRANSFER_SYNTAX_ERR)),
                                Some(employee_first_name) => {
                                    let mut employee = String::from(employee_first_name);
                                    tokens.for_each(|token| {
                                        employee.push(' ');
                                        employee.push_str(token);
                                    });
                                    Command::TransferEmployeeBetweenDepartments(
                                        employee,
                                        from_department.to_string(),
                                        to_department.to_string(),
                                    )
                                }
                            },
                            _ => Command::SyntaxErr(String::from(TRANSFER_SYNTAX_ERR)),
                        },
                    },
                },
                _ => Command::SyntaxErr(String::from(TRANSFER_SYNTAX_ERR)),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_parse_assign {
        use super::{parse_assign, Command};

        #[test]
        fn employee_name_and_department_triggers_assign() {
            let query_fragment = "Flying Tomato to Comedian";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::AssignEmployeeToDepartment(
                    "Flying Tomato".to_string(),
                    "Comedian".to_string()
                ),
                parse_assign(tokens)
            );

            let query_fragment = "Steve to Patrol";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::AssignEmployeeToDepartment("Steve".to_string(), "Patrol".to_string()),
                parse_assign(tokens)
            );
        }

        #[test]
        fn no_expression_triggers_syntax_error() {
            let query_fragment = "";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Assign\" command must specify an employee to assign and a department to assign to".to_string()),
                parse_assign(tokens)
            );
        }

        #[test]
        fn no_employee_triggers_syntax_error() {
            let query_fragment = "to Nowhere";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Assign\" command must specify an employee to assign and a department to assign to".to_string()),
                parse_assign(tokens)
            );
        }

        #[test]
        fn no_department_triggers_syntax_error() {
            let query_fragment = "Knight to";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Assign\" command must specify an employee to assign and a department to assign to".to_string()),
                parse_assign(tokens)
            );

            let query_fragment = "Bobby McBobberson to";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Assign\" command must specify an employee to assign and a department to assign to".to_string()),
                parse_assign(tokens)
            );
        }

        #[test]
        fn no_from_triggers_syntax_error() {
            let query_fragment = "Bob Accounting";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Assign\" command must specify an employee to assign and a department to assign to".to_string()),
                parse_assign(tokens)
            );

            let query_fragment = "Eldritch Horrors Closet";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Assign\" command must specify an employee to assign and a department to assign to".to_string()),
                parse_assign(tokens)
            );
        }

        #[test]
        fn multi_word_department_triggers_syntax_error() {
            let query_fragment = "Magic Missle to The Darkness";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Assign\" command must specify an employee to assign and a department to assign to".to_string()),
                parse_assign(tokens)
            );
        }
    }

    mod fn_parse_dissolve {
        use super::{parse_dissolve, Command};

        #[test]
        fn department_name_triggers_dissolve() {
            let query_fragment = "Research";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::DissolveDepartment("Research".to_string()),
                parse_dissolve(tokens)
            );
        }

        #[test]
        fn empty_name_triggers_syntax_error() {
            let query_fragment = "";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr(
                    "\"Dissolve\" command must specify a department to dissolve".to_string()
                ),
                parse_dissolve(tokens)
            );
        }

        #[test]
        fn multi_word_department_triggers_syntax_error() {
            let query_fragment = "Flight Testing";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr(
                    "Due to company policy, department names can only be one word long".to_string()
                ),
                parse_dissolve(tokens)
            );
        }
    }

    mod fn_parse_form {
        use super::{parse_form, Command};

        #[test]
        fn department_name_triggers_form() {
            let query_fragment = "Bootlegging";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::FormDepartment("Bootlegging".to_string()),
                parse_form(tokens)
            );
        }

        #[test]
        fn empty_name_triggers_syntax_error() {
            let query_fragment = "";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr(
                    "\"Form\" command must specify a department to form".to_string()
                ),
                parse_form(tokens)
            );
        }

        #[test]
        fn multi_word_department_triggers_syntax_error() {
            let query_fragment = "Cheese Wheeling";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr(
                    "Due to company policy, department names can only be one word long".to_string()
                ),
                parse_form(tokens)
            );
        }
    }

    mod fn_parse_pull {
        use super::{parse_pull, Command};

        #[test]
        fn employee_name_and_department_triggers_pull() {
            let query_fragment = "Ripe Potato from Archives";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::PullEmployeeFromDepartment(
                    "Ripe Potato".to_string(),
                    "Archives".to_string()
                ),
                parse_pull(tokens)
            );

            let query_fragment = "Steve from Patrol";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::PullEmployeeFromDepartment("Steve".to_string(), "Patrol".to_string()),
                parse_pull(tokens)
            );
        }

        #[test]
        fn no_expression_triggers_syntax_error() {
            let query_fragment = "";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Pull\" command must specify an employee to pull and a department to pull from".to_string()),
                parse_pull(tokens)
            );
        }

        #[test]
        fn no_employee_triggers_syntax_error() {
            let query_fragment = "from Nothing";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Pull\" command must specify an employee to pull and a department to pull from".to_string()),
                parse_pull(tokens)
            );
        }

        #[test]
        fn no_department_triggers_syntax_error() {
            let query_fragment = "Jones from";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Pull\" command must specify an employee to pull and a department to pull from".to_string()),
                parse_pull(tokens)
            );

            let query_fragment = "Bobby McBobberson from";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Pull\" command must specify an employee to pull and a department to pull from".to_string()),
                parse_pull(tokens)
            );
        }

        #[test]
        fn no_from_triggers_syntax_error() {
            let query_fragment = "Bob Accounting";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Pull\" command must specify an employee to pull and a department to pull from".to_string()),
                parse_pull(tokens)
            );

            let query_fragment = "Eldritch Horrors Closet";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Pull\" command must specify an employee to pull and a department to pull from".to_string()),
                parse_pull(tokens)
            );
        }

        #[test]
        fn multi_word_department_triggers_syntax_error() {
            let query_fragment = "Tony from The Darkness";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Pull\" command must specify an employee to pull and a department to pull from".to_string()),
                parse_pull(tokens)
            );
        }
    }

    mod fn_parse_show {
        use super::{parse_show, Command};

        #[test]
        fn departments_triggers_show() {
            let query_fragment = "departments";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(Command::ShowDepartments, parse_show(tokens));
        }

        #[test]
        fn no_expression_triggers_syntax_error() {
            let query_fragment = "";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Show\" command must specify a list name".to_string()),
                parse_show(tokens)
            );
        }

        #[test]
        fn other_list_triggers_syntax_error() {
            let query_fragment = "bunnies";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("Cannot show \"bunnies\": list does not exist".to_string()),
                parse_show(tokens)
            );
        }

        #[test]
        fn multi_word_list_triggers_syntax_error() {
            let query_fragment = "departments flotsam";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr(
                    "Unexpected token \"flotsam\" after list name \"departments\"".to_string()
                ),
                parse_show(tokens)
            );
        }
    }

    mod fn_parse_transfer {
        use super::{parse_transfer, Command};

        #[test]
        fn employee_name_and_departments_trigger_transfer() {
            let query_fragment = "Hot Potato from Susie to Micky";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::TransferEmployeeBetweenDepartments(
                    "Hot Potato".to_string(),
                    "Susie".to_string(),
                    "Micky".to_string()
                ),
                parse_transfer(tokens)
            );

            let query_fragment = "Girl from Uptown to Downtown";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::TransferEmployeeBetweenDepartments(
                    "Girl".to_string(),
                    "Uptown".to_string(),
                    "Downtown".to_string()
                ),
                parse_transfer(tokens)
            );
        }

        #[test]
        fn no_expression_triggers_syntax_error() {
            let query_fragment = "";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );
        }

        #[test]
        fn no_employee_triggers_syntax_error() {
            let query_fragment = "from Nothing to Everything";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );
        }

        #[test]
        fn no_from_department_triggers_syntax_error() {
            let query_fragment = "Flare from to Sol";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );

            let query_fragment = "Bobby McBobberson to Staging";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );
        }

        #[test]
        fn no_to_department_triggers_syntax_error() {
            let query_fragment = "Bones from Grimdiana";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );

            let query_fragment = "Bobby McBobberson from South to";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );
        }

        #[test]
        fn no_from_triggers_syntax_error() {
            let query_fragment = "Bob Accounting to Editing";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );
        }

        #[test]
        fn no_to_triggers_syntax_error() {
            let query_fragment = "Bob from Accounting Editing";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );
        }

        #[test]
        fn multi_word_department_triggers_syntax_error() {
            let query_fragment = "Tony from The Darkness to Light";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );

            let query_fragment = "Tony from Dark to The Lightness";
            let tokens = query_fragment.split_whitespace();

            assert_eq!(
                Command::SyntaxErr("\"Transfer\" command must specify an employee, a department to transfer from, and a department to transfer to".to_string()),
                parse_transfer(tokens)
            );
        }
    }
}
