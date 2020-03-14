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

#[test]
fn user_can_form_departments() {
    let mut db = Database::new();

    match db.query("form Yesteryears".to_string()) {
        QueryResponse::Message(message) => {
            assert_eq!("Formed \"Yesteryears\" department", message);
        }
        _ => panic!(),
    }

    match db.query("show departments".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(1, table.data.len());

            let department_row = &table.data[0];
            assert_eq!("Yesteryears", department_row.get("Department").unwrap());
        }
        _ => panic!(),
    }
}

#[test]
fn user_can_show_departments_alphabetically() {
    let mut db = Database::new();

    db.query("form waffles".to_string());
    db.query("form pancakes".to_string());
    db.query("form scrambles".to_string());
    db.query("form sunnies".to_string());
    db.query("form boileds".to_string());
    db.query("form poacheds".to_string());

    match db.query("show departments".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(1, table.headers.len());

            let header_name = "Department".to_string();
            assert_eq!(header_name, table.headers[0]);

            assert_eq!(6, table.data.len());

            let departments = table
                .data
                .iter()
                .map(|row| row.get(&header_name).unwrap().to_owned())
                .collect::<Vec<String>>();
            assert_eq!(
                vec![
                    "Boileds".to_string(),
                    "Pancakes".to_string(),
                    "Poacheds".to_string(),
                    "Scrambles".to_string(),
                    "Sunnies".to_string(),
                    "Waffles".to_string(),
                ],
                departments
            );
        }
        _ => panic!(),
    }
}

#[test]
fn user_can_dissolve_departments() {
    let mut db = Database::new();

    db.query("form finances".to_string());

    match db.query("show departments".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(1, table.data.len());
        }
        _ => panic!(),
    }

    match db.query("dissolve finances".to_string()) {
        QueryResponse::Message(message) => {
            assert_eq!("Dissolved \"Finances\" department".to_string(), message);
        }
        _ => panic!(),
    }

    match db.query("show departments".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(0, table.data.len());
        }
        _ => panic!(),
    }
}

#[test]
fn user_can_assign_employees_to_departments() {
    let mut db = Database::new();

    db.query("form hr".to_string());

    match db.query("list employees in hr".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(0, table.data.len());
        }
        _ => panic!(),
    }

    match db.query("assign margaret to hr".to_string()) {
        QueryResponse::Message(message) => {
            assert_eq!(
                "Assigned employee \"Margaret\" to Hr department".to_string(),
                message
            );
        }
        _ => panic!(),
    }

    match db.query("list employees in hr".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(1, table.data.len());

            assert_eq!("Margaret", table.data[0].get("Employee").unwrap());
        }
        _ => panic!(),
    }
}

#[test]
fn user_can_list_all_employees_alphabetically() {
    let mut db = Database::new();

    db.query("form parents".to_string());
    db.query("form kids".to_string());

    db.query("assign mommy to parents".to_string());
    db.query("assign daddy to parents".to_string());
    db.query("assign brother to kids".to_string());
    db.query("assign sister to kids".to_string());

    match db.query("list employees".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(4, table.data.len());

            assert_eq!(
                vec!["Brother", "Daddy", "Mommy", "Sister",],
                table
                    .data
                    .iter()
                    .map(|row| row.get("Employee").unwrap())
                    .collect::<Vec<&String>>()
            );
        }
        _ => panic!(),
    }
}

#[test]
fn user_can_list_all_employees_alphabetically_grouped_by_department_alphabetically() {
    let mut db = Database::new();

    db.query("form parents".to_string());
    db.query("form kids".to_string());

    db.query("assign mommy to parents".to_string());
    db.query("assign daddy to parents".to_string());
    db.query("assign brother to kids".to_string());
    db.query("assign sister to kids".to_string());

    match db.query("list employees by department".to_string()) {
        QueryResponse::Table(table) => {
            assert_eq!(4, table.data.len());

            assert_eq!(
                vec![
                    ("Kids".to_string(), "Brother".to_string()),
                    ("Kids".to_string(), "Sister".to_string()),
                    ("Parents".to_string(), "Daddy".to_string()),
                    ("Parents".to_string(), "Mommy".to_string()),
                ],
                table
                    .data
                    .iter()
                    .map(|row| (
                        row.get("Department").unwrap().to_owned(),
                        row.get("Employee").unwrap().to_owned()
                    ))
                    .collect::<Vec<(String, String)>>()
            );
        }
        _ => panic!(),
    }
}

// #[test]
// fn user_can_list_all_employees_alphabetically_in_a_department() {}

// #[test]
// fn user_can_transfer_employees_between_departments() {}

// #[test]
// fn user_can_pull_employees_from_departments() {}
