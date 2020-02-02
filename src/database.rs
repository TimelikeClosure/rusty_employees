use std::collections::HashMap;

mod commands;
use commands::Command;
mod errors;
use errors::QueryError;
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
                let departments = self.store.departments().list();
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
            Command::FormDepartment(department) => {
                match self.store.departments().create(&department) {
                    Ok(department) => QueryResponse::Message(format!("Formed \"{}\" department", department)),
                    Err(query_error) => format_query_error(query_error),
                }
            },
            Command::ListEmployees => {
                let departments = self.store
                    .departments()
                    .list();
                let department_employee_groups = departments
                    .iter()
                    .map(|department_name| {(
                        department_name.to_owned(),
                        self.store
                            .department(department_name.as_str())
                            .unwrap()
                            .employees()
                            .list()
                    )})
                    .collect::<Vec<(String, Vec<String>)>>();
                let mut employees: Vec<String> = Vec::new();
                for (_department_name, employee_list) in department_employee_groups {
                    for employee_name in employee_list {
                        employees.push(employee_name.to_owned());
                    }
                }
                employees.sort_unstable_by_key(|name| name.to_uppercase().to_owned());
                let employees = employees;
                const COLUMN_NAME: &str = "Employee";
                QueryResponse::Table(
                    Table {
                        title: String::from("Showing all Employees"),
                        headers: vec![COLUMN_NAME.to_string()],
                        data: employees.iter().map(|employee| {
                            let mut row = HashMap::new();
                            row.insert(COLUMN_NAME.to_string(), employee.to_owned());
                            row
                        }).fold(Vec::new(), |mut rows, row| {
                            rows.push(row);
                            rows
                        })
                    }
                )
            },
            Command::ListEmployeesByDepartment => {
                let departments = self.store
                    .departments()
                    .list();
                let department_employee_groups = departments
                    .iter()
                    .map(|department_name| {(
                        department_name.to_owned(),
                        self.store
                            .department(department_name.as_str())
                            .unwrap()
                            .employees()
                            .list()
                    )})
                    .collect::<Vec<(String, Vec<String>)>>();
                let mut department_employees: Vec<(String, String)> = Vec::new();
                for (department_name, employees) in department_employee_groups {
                    for employee_name in employees {
                        department_employees.push((department_name.to_owned(), employee_name.to_owned()));
                    }
                }
                let department_employees = department_employees;
                const COLUMN_NAMES: [&str; 2] = ["Department", "Employee"];
                QueryResponse::Table(
                    Table {
                        title: String::from("Showing Employees grouped by Department"),
                        headers: vec![COLUMN_NAMES[0].to_string(), COLUMN_NAMES[1].to_string()],
                        data: department_employees.iter().map(|(department_name, employee_name)| {
                            let mut row = HashMap::new();
                            row.insert(COLUMN_NAMES[0].to_string(), department_name.to_owned());
                            row.insert(COLUMN_NAMES[1].to_string(), employee_name.to_owned());
                            row
                        }).fold(Vec::new(), |mut rows, row| {
                            rows.push(row);
                            rows
                        })
                    }
                )
            },
            Command::ListEmployeesInDepartment(department_name) => {
                match self.store.department(&department_name) {
                    Ok(department) => {
                        let employees = department.employees().list();
                        const COLUMN_NAME: &str = "Employee";
                        QueryResponse::Table(
                            Table {
                                title: format!("Showing Employees assigned to the {} Department", department.name()),
                                headers: vec![COLUMN_NAME.to_string()],
                                data: employees.iter().map(|employee_name| {
                                    let mut row = HashMap::new();
                                    row.insert(COLUMN_NAME.to_string(), employee_name.to_owned());
                                    row
                                }).fold(Vec::new(), |mut rows, row| {
                                    rows.push(row);
                                    rows
                                })
                            }
                        )
                    },
                    Err(query_error) => format_query_error(query_error),
                }
            },
            Command::AssignEmployeeToDepartment(employee_name, department_name) => {
                match self.store.department(&department_name) {
                    Ok(department) => match department.employees().create(&employee_name) {
                        Ok(employee) => QueryResponse::Message(format!(
                            "Assigned employee \"{}\" to {} department",
                            employee, department_name
                        )),
                        Err(query_error) => format_query_error(query_error),
                    },
                    Err(query_error) => format_query_error(query_error),
                }
            },
            Command::TransferEmployeeBetweenDepartments(employee_name, from_department_name, to_department_name) => {
                if from_department_name == to_department_name {
                    return QueryResponse::Message(String::from("ERROR: Cannot move employee from department to same department"));
                }
                match self.store.department(&from_department_name) {
                    Err(query_error) => return format_query_error(query_error),
                    Ok(from_department) => {
                        if let Err(query_error) = from_department.employees().employee(&employee_name) {
                            return format_query_error(query_error);
                        }
                    },
                };
                match self.store.department(&to_department_name) {
                    Err(query_error) => return format_query_error(query_error),
                    Ok(to_department) => {
                        if let Err(_) = to_department.employees().create(&employee_name) {
                            return format_query_error(QueryError::Conflict(format!(
                                "Employee \"{}\" already exists in department \"{}\"",
                                employee_name, to_department_name
                            )));
                        }
                    },
                };
                self.store.department(&from_department_name).unwrap().employees().delete(&employee_name).unwrap();
                QueryResponse::Message(format!(
                    "Employee \"{}\" transferred from \"{}\" to \"{}\" department",
                    employee_name, from_department_name, to_department_name
                ))
            },
            Command::PullEmployeeFromDepartment(employee_name, department_name) => {
                match self.store.department(&department_name) {
                    Ok(department) => {
                        match department.employees().delete(&employee_name) {
                            Err(query_error) => format_query_error(query_error),
                            Ok(_) => QueryResponse::Message(format!(
                                "Pulled employee \"{}\" from department \"{}\"",
                                employee_name, department_name
                            )),
                        }
                    },
                    Err(query_error) => format_query_error(query_error),
                }
            },
            Command::DissolveDepartment(department_name) => {
                match self.store.departments().delete(&department_name) {
                    Ok(_) => QueryResponse::Message(format!(
                        "Dissolved \"{}\" department", department_name
                    )),
                    Err(query_error) => format_query_error(query_error),
                }
            },
        }
    }
}

fn format_query_error(error: QueryError) -> QueryResponse {
    use QueryResponse::Message;
    match error {
        QueryError::Conflict(message) => Message(format!("ERROR: Query conflict: {}", message)),
        QueryError::NotFound(message) => {
            Message(format!("ERROR: Query target not found: {}", message))
        }
    }
}
