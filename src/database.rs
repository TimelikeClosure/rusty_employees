//! In-memory departmental employee database with SQL-like query parsing.
use std::collections::HashMap;

mod commands;
use commands::Command;
mod errors;
use errors::QueryError;
mod store;
use store::Store;

/// Unformatted tabular data.
#[derive(Debug, PartialEq)]
pub struct Table {
    /// Data set name.
    pub title: String,
    /// Column names / header labels, in default order.
    pub headers: Vec<String>,
    /// Row data, with each row containing a map of data by column name.
    pub data: Vec<HashMap<String, String>>,
}

/// Standardized query result output formats
#[derive(Debug, PartialEq)]
pub enum QueryResponse {
    /// Stop listening for queries
    Exit,
    /// No operation was performed
    NoOp,
    /// String message output
    Message(String),
    /// Tabular data output
    Table(Table),
}

/// Departmental employee database with data store and SQL-like query parsing
#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Database {
    store: Store,
}

impl Database {
    /// Creates a new empty database.
    ///
    /// Creates a new database for tracking employees by department. The database
    /// will be empty initially. It is recommended the database be kept mutable
    /// as long as there is any possibility of writing to it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use employees::database::Database;
    ///
    /// let mut db = Database::new();
    /// ```
    pub fn new() -> Self {
        Database {
            store: Store::new(),
        }
    }

    /// Seeds a database with some dummy data.
    ///
    /// Often times it's easier to develop with a pre-seeded database. To facilitate that,
    /// the `.seed()` method is available to pre-fill the database with a modest data set.
    ///
    /// # Panics
    ///
    /// The `.seed()` method will panic if any of the generated departments already exist in the data store.
    /// Since the seed data generated by calling `.seed()` is always the same, calling `.seed()`
    /// twice will cause a panic:
    /// ```should_panic
    /// use employees::database::Database;
    ///
    /// let mut db = Database::new();
    /// db.seed();
    /// db.seed(); // Panics at this point.
    /// ```
    ///
    /// # Examples
    ///
    /// To create a seeded database, create a new mutable database, then call the `.seed()` method on it:
    /// ```rust
    /// use employees::database::Database;
    ///
    /// let mut db = Database::new();
    /// db.seed();
    /// ```
    /// It is recommended to do this before any other data is inserted to avoid conflicts.
    pub fn seed(&mut self) {
        self.store.seed()
    }

    /// Perform a query on the database
    ///
    /// # Examples
    ///
    /// ## Helper Commands
    ///
    /// Some queries are used for helper level operations and don't touch any stored data.
    ///
    /// The `"exit"` query can be used to trigger an `Exit` response:
    /// ```rust
    /// use employees::database::{Database, QueryResponse};
    ///
    /// let mut db = Database::new();
    /// assert_eq!(
    ///   db.query("Exit".to_string()),
    ///   QueryResponse::Exit
    /// );
    /// ```
    /// This can be used to signal any outside code to break out of a REPL, or something similar.
    ///
    /// The `"help"` query can be used to get a list of available queries.
    /// ```rust
    /// use employees::database::{Database, QueryResponse};
    ///
    /// let mut db = Database::new();
    /// db.query("Help".to_string());
    /// ```
    ///
    /// ## Query Errors
    ///
    /// Invalid queries will trigger different responses, depending on why they're considered invalid.
    ///
    /// If a query contains nothing but whitespace, `.query()` will respond with a `NoOp`:
    /// ```rust
    /// use employees::database::{Database, QueryResponse};
    ///
    /// let mut db = Database::new();
    /// assert_eq!(
    ///   db.query("   ".to_string()),
    ///   QueryResponse::NoOp
    /// );
    /// ```
    ///
    /// If a query begins with an invalid command keyword, `.query()` will respond with an invalid command message:
    /// ```rust
    /// use employees::database::{Database, QueryResponse};
    ///
    /// let mut db = Database::new();
    /// assert_eq!(
    ///   db.query("get waffles".to_string()),
    ///   QueryResponse::Message(
    ///     "ERROR: Invalid command \"get\". Please check your spelling, \
    ///      or type \"Help\" for the list of available commands"
    ///     .to_string()
    ///   )
    /// );
    /// ```
    ///
    /// Otherwise, if the query syntax is invalid in some other way, `.query()` will respond with a command-specific message:
    /// ```rust
    /// use employees::database::{Database, QueryResponse};
    ///
    /// let mut db = Database::new();
    /// assert_eq!(
    ///   db.query("list waffles".to_string()),
    ///   QueryResponse::Message(
    ///     "ERROR: Invalid command syntax: Cannot list \"waffles\": list does not exist"
    ///     .to_string()
    ///   )
    /// );
    /// ```
    ///
    /// ## Departments
    ///
    /// Departments can be viewed and edited with the `"list"`, `"create"`, and `"delete"` query commands. Departments must be one word long.
    /// ```rust
    /// use std::collections::HashMap;
    /// use employees::database::{Database, QueryResponse, Table};
    ///
    /// let mut db = Database::new();
    ///
    /// assert_eq!(
    ///   db.query("form sales".to_string()),
    ///   QueryResponse::Message("Formed \"Sales\" department".to_string())
    /// );
    ///
    /// assert_eq!(
    ///   db.query("show departments".to_string()),
    ///   QueryResponse::Table(Table {
    ///     title: "Showing all Departments".to_string(),
    ///     headers: vec!["Department".to_string()],
    ///     data: vec![{
    ///       let mut data = HashMap::new();
    ///       data.insert("Department".to_string(), "Sales".to_string());
    ///       data
    ///     }]
    ///   })
    /// );
    ///
    /// assert_eq!(
    ///   db.query("dissolve sales".to_string()),
    ///   QueryResponse::Message("Dissolved \"Sales\" department".to_string())
    /// );
    /// #
    /// # assert_eq!(
    /// #   db.query("show departments".to_string()),
    /// #   QueryResponse::Table(Table {
    /// #     title: "Showing all Departments".to_string(),
    /// #     headers: vec!["Department".to_string()],
    /// #     data: vec![]
    /// #   })
    /// # );
    /// ```
    ///
    /// ## Employees
    ///
    /// Employees can be viewed and edited with the `"list"`, `"assign"`, `"transfer"`, and `"pull"` query commands.
    /// The `"list"` command can be used to show all employees on their own or group / filter by department.
    ///
    /// ```rust
    /// # use std::collections::HashMap;
    /// # use employees::database::{Database, QueryResponse, Table};
    /// #
    /// # let mut db = Database::new();
    /// #
    /// # db.query("form shipping".to_string());
    /// # db.query("form receiving".to_string());
    /// #
    /// db.query("assign baby driver to shipping".to_string());
    /// db.query("assign the blob to receiving".to_string());
    /// db.query("assign portal to receiving".to_string());
    /// db.query("assign portal to shipping".to_string());
    ///
    /// assert_eq!(
    ///   db.query("list employees".to_string()),
    ///   QueryResponse::Table(Table {
    ///     title: "Showing all Employees".to_string(),
    ///     headers: vec!["Employee".to_string()],
    ///     data: vec![
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Employee".to_string(), "Baby Driver".to_string());
    ///         data
    ///       },
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Employee".to_string(), "Portal".to_string());
    ///         data
    ///       },
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Employee".to_string(), "Portal".to_string());
    ///         data
    ///       },
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Employee".to_string(), "The Blob".to_string());
    ///         data
    ///       },
    ///     ]
    ///   })
    /// );
    ///
    /// assert_eq!(
    ///   db.query("list employees by department".to_string()),
    ///   QueryResponse::Table(Table {
    ///     title: "Showing Employees grouped by Department".to_string(),
    ///     headers: vec!["Department".to_string(), "Employee".to_string()],
    ///     data: vec![
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Department".to_string(), "Receiving".to_string());
    ///         data.insert("Employee".to_string(), "Portal".to_string());
    ///         data
    ///       },
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Department".to_string(), "Receiving".to_string());
    ///         data.insert("Employee".to_string(), "The Blob".to_string());
    ///         data
    ///       },
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Department".to_string(), "Shipping".to_string());
    ///         data.insert("Employee".to_string(), "Baby Driver".to_string());
    ///         data
    ///       },
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Department".to_string(), "Shipping".to_string());
    ///         data.insert("Employee".to_string(), "Portal".to_string());
    ///         data
    ///       },
    ///     ]
    ///   })
    /// );
    ///
    /// db.query("transfer the blob from receiving to shipping".to_string());
    /// db.query("pull baby driver from shipping".to_string());
    ///
    /// assert_eq!(
    ///   db.query("list employees in shipping".to_string()),
    ///   QueryResponse::Table(Table {
    ///     title: "Showing Employees assigned to the Shipping Department".to_string(),
    ///     headers: vec!["Employee".to_string()],
    ///     data: vec![
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Employee".to_string(), "Portal".to_string());
    ///         data
    ///       },
    ///       {
    ///         let mut data = HashMap::new();
    ///         data.insert("Employee".to_string(), "The Blob".to_string());
    ///         data
    ///       },
    ///     ]
    ///   })
    /// );
    /// ```
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
            Command::ShowDepartments => self.list_departments(),
            Command::FormDepartment(department_name) => {
                self.create_department(department_name)
            },
            Command::ListEmployees => {
                self.list_employees()
            },
            Command::ListEmployeesByDepartment => {
                self.list_employees_by_department()
            },
            Command::ListEmployeesInDepartment(department_name) => {
                self.list_employees_in_department(department_name)
            },
            Command::AssignEmployeeToDepartment(employee_name, department_name) => self.create_employee(employee_name, department_name),
            Command::TransferEmployeeBetweenDepartments(employee_name, from_department_name, to_department_name) => {
                self.move_employee(employee_name, from_department_name, to_department_name)
            },
            Command::PullEmployeeFromDepartment(employee_name, department_name) => {
                self.delete_employee(employee_name, department_name)
            },
            Command::DissolveDepartment(department_name) => self.delete_department(department_name),
        }
    }

    fn create_department(&mut self, department_name: String) -> QueryResponse {
        match self.store.departments_mut().create(&department_name) {
            Ok(department) => {
                QueryResponse::Message(format!("Formed \"{}\" department", department))
            }
            Err(query_error) => format_query_error(query_error),
        }
    }

    fn create_employee(&mut self, employee_name: String, department_name: String) -> QueryResponse {
        match self.store.department_mut(&department_name) {
            Ok(department) => match department.employees_mut().create(&employee_name) {
                Ok(employee) => QueryResponse::Message(format!(
                    "Assigned employee \"{}\" to {} department",
                    employee,
                    department.name()
                )),
                Err(query_error) => format_query_error(query_error),
            },
            Err(query_error) => format_query_error(query_error),
        }
    }

    fn delete_department(&mut self, department_name: String) -> QueryResponse {
        match self.store.departments_mut().delete(&department_name) {
            Ok(department) => {
                QueryResponse::Message(format!("Dissolved \"{}\" department", department))
            }
            Err(query_error) => format_query_error(query_error),
        }
    }

    fn delete_employee(&mut self, employee_name: String, department_name: String) -> QueryResponse {
        match self.store.department_mut(&department_name) {
            Ok(department) => match department.employees_mut().delete(&employee_name) {
                Err(query_error) => format_query_error(query_error),
                Ok(_) => QueryResponse::Message(format!(
                    "Pulled employee \"{}\" from department \"{}\"",
                    employee_name, department_name
                )),
            },
            Err(query_error) => format_query_error(query_error),
        }
    }

    fn list_departments(&self) -> QueryResponse {
        let departments = self.store.departments().list();
        const COLUMN_NAME: &str = "Department";
        QueryResponse::Table(Table {
            title: String::from("Showing all Departments"),
            headers: vec![COLUMN_NAME.to_string()],
            data: departments
                .iter()
                .map(|dept_name| {
                    let mut row = HashMap::new();
                    row.insert(COLUMN_NAME.to_string(), dept_name.to_owned());
                    row
                })
                .fold(Vec::new(), |mut rows, row| {
                    rows.push(row);
                    rows
                }),
        })
    }

    fn list_employees(&self) -> QueryResponse {
        let departments = self.store.departments().list();
        let department_employee_groups = departments
            .iter()
            .map(|department_name| {
                (
                    department_name.to_owned(),
                    self.store
                        .department(department_name.as_str())
                        .unwrap()
                        .employees()
                        .list(),
                )
            })
            .collect::<Vec<(String, Vec<String>)>>();
        let mut employees: Vec<String> = Vec::new();
        for (_department_name, employee_list) in department_employee_groups {
            for employee_name in employee_list {
                employees.push(employee_name.to_owned());
            }
        }
        employees.sort_by_key(|name| name.to_uppercase());
        let employees = employees;
        const COLUMN_NAME: &str = "Employee";
        QueryResponse::Table(Table {
            title: String::from("Showing all Employees"),
            headers: vec![COLUMN_NAME.to_string()],
            data: employees
                .iter()
                .map(|employee| {
                    let mut row = HashMap::new();
                    row.insert(COLUMN_NAME.to_string(), employee.to_owned());
                    row
                })
                .fold(Vec::new(), |mut rows, row| {
                    rows.push(row);
                    rows
                }),
        })
    }

    fn list_employees_by_department(&self) -> QueryResponse {
        let departments = self.store.departments().list();
        let department_employee_groups = departments
            .iter()
            .map(|department_name| {
                (
                    department_name.to_owned(),
                    self.store
                        .department(department_name.as_str())
                        .unwrap()
                        .employees()
                        .list(),
                )
            })
            .collect::<Vec<(String, Vec<String>)>>();
        let mut department_employees: Vec<(String, String)> = Vec::new();
        for (department_name, employees) in department_employee_groups {
            for employee_name in employees {
                department_employees.push((department_name.to_owned(), employee_name.to_owned()));
            }
        }
        let department_employees = department_employees;
        const COLUMN_NAMES: [&str; 2] = ["Department", "Employee"];
        QueryResponse::Table(Table {
            title: String::from("Showing Employees grouped by Department"),
            headers: vec![COLUMN_NAMES[0].to_string(), COLUMN_NAMES[1].to_string()],
            data: department_employees
                .iter()
                .map(|(department_name, employee_name)| {
                    let mut row = HashMap::new();
                    row.insert(COLUMN_NAMES[0].to_string(), department_name.to_owned());
                    row.insert(COLUMN_NAMES[1].to_string(), employee_name.to_owned());
                    row
                })
                .fold(Vec::new(), |mut rows, row| {
                    rows.push(row);
                    rows
                }),
        })
    }

    fn list_employees_in_department(&self, department_name: String) -> QueryResponse {
        match self.store.department(&department_name) {
            Ok(department) => {
                let employees = department.employees().list();
                const COLUMN_NAME: &str = "Employee";
                QueryResponse::Table(Table {
                    title: format!(
                        "Showing Employees assigned to the {} Department",
                        department.name()
                    ),
                    headers: vec![COLUMN_NAME.to_string()],
                    data: employees
                        .iter()
                        .map(|employee_name| {
                            let mut row = HashMap::new();
                            row.insert(COLUMN_NAME.to_string(), employee_name.to_owned());
                            row
                        })
                        .fold(Vec::new(), |mut rows, row| {
                            rows.push(row);
                            rows
                        }),
                })
            }
            Err(query_error) => format_query_error(query_error),
        }
    }

    fn move_employee(
        &mut self,
        employee_name: String,
        from_department_name: String,
        to_department_name: String,
    ) -> QueryResponse {
        if from_department_name == to_department_name {
            return QueryResponse::Message(String::from(
                "ERROR: Cannot move employee from department to same department",
            ));
        }
        match self.store.department(&from_department_name) {
            Err(query_error) => return format_query_error(query_error),
            Ok(from_department) => {
                if let Err(query_error) = from_department.employees().employee(&employee_name) {
                    return format_query_error(query_error);
                }
            }
        };
        match self.store.department_mut(&to_department_name) {
            Err(query_error) => return format_query_error(query_error),
            Ok(to_department) => {
                if to_department
                    .employees_mut()
                    .create(&employee_name)
                    .is_err()
                {
                    return format_query_error(QueryError::Conflict(format!(
                        "Employee \"{}\" already exists in department \"{}\"",
                        employee_name, to_department_name
                    )));
                }
            }
        };
        self.store
            .department_mut(&from_department_name)
            .unwrap()
            .employees_mut()
            .delete(&employee_name)
            .unwrap();
        QueryResponse::Message(format!(
            "Employee \"{}\" transferred from \"{}\" to \"{}\" department",
            employee_name, from_department_name, to_department_name
        ))
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

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use super::*;

        mod seed {
            use super::Database;

            #[test]
            fn seed_adds_content_to_database() {
                let mut db = Database::new();
                db.seed();
                assert_ne!(0, db.store.departments().list().len());
            }
        }

        mod query {
            use super::{Database, QueryResponse};

            #[test]
            fn runs_a_query() {
                let mut db = Database::new();

                assert_eq!(QueryResponse::NoOp, db.query("".to_string()));
            }
        }
    }

    mod fn_format_query_error {
        use super::{format_query_error, QueryError, QueryResponse};

        #[test]
        fn conflict() {
            assert_eq!(
                QueryResponse::Message("ERROR: Query conflict: How are you?".to_string()),
                format_query_error(QueryError::Conflict("How are you?".to_string()))
            );
        }

        #[test]
        fn not_found() {
            assert_eq!(
                QueryResponse::Message("ERROR: Query target not found: I found it!".to_string()),
                format_query_error(QueryError::NotFound("I found it!".to_string()))
            )
        }
    }
}
