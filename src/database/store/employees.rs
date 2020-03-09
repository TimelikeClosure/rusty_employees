use super::super::errors::QueryError;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Employee {
    name: String,
}

impl Employee {
    pub fn new(name: &str) -> Self {
        Employee {
            name: to_name(name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Employees {
    index: BTreeMap<String, Employee>,
}

impl Employees {
    pub fn new() -> Self {
        Employees {
            index: BTreeMap::new(),
        }
    }

    pub fn employee(&self, employee_name: &str) -> Result<&Employee, QueryError> {
        match self.index.get(&to_key(employee_name)) {
            None => Err(QueryError::NotFound(format!(
                "Employee \"{}\" does not exist",
                employee_name
            ))),
            Some(employee) => Ok(employee),
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.index
            .iter()
            .map(|(_key, employee)| employee.name().to_string())
            .collect::<Vec<String>>()
    }

    pub fn create(&mut self, employee: &str) -> Result<String, QueryError> {
        match self.index.entry(to_key(employee)) {
            Entry::Vacant(entry) => {
                entry.insert(Employee::new(employee));
                Ok(to_name(employee))
            }
            Entry::Occupied(_) => Err(QueryError::Conflict(format!(
                "Employee \"{}\" already exists",
                employee,
            ))),
        }
    }

    pub fn delete(&mut self, employee: &str) -> Result<(), QueryError> {
        match self.index.remove(&to_key(employee)) {
            None => Err(QueryError::NotFound(format!(
                "Employee \"{}\" not found",
                employee
            ))),
            Some(_) => Ok(()),
        }
    }
}

fn to_key(value: &str) -> String {
    value.to_uppercase()
}

fn to_name(value: &str) -> String {
    value
        .split_whitespace()
        .map(|word| {
            word.chars()
                .enumerate()
                .map(|(index, character)| {
                    if index == 0 {
                        character.to_uppercase().next().unwrap()
                    } else {
                        character.to_lowercase().next().unwrap()
                    }
                })
                .collect::<String>()
        })
        .enumerate()
        .fold(String::new(), |mut name, (index, word)| {
            if index > 0 {
                name.push(' ');
            }
            name.push_str(&word);
            name
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_to_key {
        use super::to_key;

        #[test]
        fn capitalizes_all_letters() {
            assert_eq!("ANGRY BOB".to_string(), to_key("Angry Bob"));
        }
    }

    mod fn_to_name {
        use super::to_name;

        #[test]
        fn capitalizes_each_word() {
            assert_eq!("Greek Salad".to_string(), to_name("gReEk SaLaD"));
        }
    }

    mod employee {
        use super::{to_name, Employee};

        #[test]
        fn name_returns_name() {
            let employee = Employee::new("Joe Mombo");

            assert_eq!(to_name("Joe Mombo"), employee.name());
        }
    }

    mod employees {
        use super::*;

        mod employee {
            use super::{Employee, Employees, QueryError};

            #[test]
            fn employee_exists() {
                let mut employees = Employees::new();
                employees.create("James McGregor").unwrap();

                assert_eq!(
                    Ok(&(Employee::new("James McGregor"))),
                    employees.employee("James McGregor")
                );
            }

            #[test]
            fn employee_doesnt_exist() {
                assert_eq!(
                    Err(QueryError::NotFound(
                        "Employee \"Slenderman\" does not exist".to_string()
                    )),
                    Employees::new().employee("Slenderman")
                );
            }
        }

        mod list {
            use super::Employees;

            #[test]
            fn gets_employee_names() {
                let mut employees = Employees::new();
                employees.create("Sally Simmerman").unwrap();
                employees.create("Jose Schwartz").unwrap();
                employees.create("Yun Balloon").unwrap();

                assert_eq!(
                    vec!["Jose Schwartz", "Sally Simmerman", "Yun Balloon"],
                    employees.list()
                );
            }
        }

        mod create {
            use super::Employees;

            #[test]
            fn adds_employee() {
                let mut employees = Employees::new();

                employees.create("Cheese Wheelin").unwrap();
                assert_eq!(vec!["Cheese Wheelin"], employees.list());

                employees.create("Gouda Pest").unwrap();
                assert_eq!(vec!["Cheese Wheelin", "Gouda Pest"], employees.list());
            }

            #[test]
            fn fails_on_duplicate_key() {
                let mut employees = Employees::new();
                employees.create("John Doe").unwrap();
                assert_eq!(vec!["John Doe"], employees.list());

                employees.create("John Doe").unwrap_err();
                assert_eq!(vec!["John Doe"], employees.list());
            }
        }

        mod delete {}
    }
}
