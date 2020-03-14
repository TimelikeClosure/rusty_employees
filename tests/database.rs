use employees::database::{Database, QueryResponse};

#[test]
fn user_can_exit() {
    let mut db = Database::new();

    assert_eq!(QueryResponse::Exit, db.query("exit".to_string()));
}

#[test]
fn user_can_see_help() {
    use std::collections::HashSet;

    let mut db = Database::new();

    let response: QueryResponse = db.query("help".to_string());
    match response {
        QueryResponse::Message(message) => {
            let mut required_commands = [
                "ASSIGN {EMPLOYEE} TO {DEPARTMENT}",
                "DISSOLVE {DEPARTMENT}",
                "EXIT",
                "FORM {DEPARTMENT}",
                "HELP",
                "LIST EMPLOYEES",
                "LIST EMPLOYEES BY DEPARTMENT",
                "LIST EMPLOYEES IN {DEPARTMENT}",
                "PULL {EMPLOYEE} FROM {DEPARTMENT}",
                "SHOW DEPARTMENTS",
                "TRANSFER {EMPLOYEE} FROM {DEPARTMENT} TO {DEPARTMENT}",
            ]
            .iter()
            .map(|command| {
                let mut command_string = "\"".to_owned();
                command_string.push_str(command);
                command_string.push('"');
                command_string
            })
            .collect::<HashSet<String>>();

            message.to_uppercase().lines().for_each(|line| {
                let mut command = None;
                for command_string in required_commands.iter() {
                    if line[..].contains(command_string) {
                        command = Some(command_string);
                        break;
                    }
                }
                if let Some(command_string) = command {
                    let command_string = command_string.to_owned();
                    required_commands.remove(&command_string);
                }
            });

            assert_eq!(
                0,
                required_commands.len(),
                "Missing required commands: {:?}",
                required_commands
            );
        }
        _ => panic!(),
    };
}

// #[test]
// fn user_can_form_departments() {}

// #[test]
// fn user_can_show_departments() {}

// #[test]
// fn user_can_dissolve_departments() {}

// #[test]
// fn user_can_assign_employees_to_departments() {}

// #[test]
// fn user_can_list_all_employees_alphabetically() {}

// #[test]
// fn user_can_list_all_employees_alphabetically_grouped_by_department_alphabetically() {}

// #[test]
// fn user_can_list_all_employees_alphabetically_in_a_department() {}

// #[test]
// fn user_can_transfer_employees_between_departments() {}

// #[test]
// fn user_can_pull_employees_from_departments() {}
