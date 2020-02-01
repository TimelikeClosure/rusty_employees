use super::super::errors::QueryError;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Employee {
    name: String,
}

impl Employee {
    pub fn new(name: &str) -> Employee {
        Employee {
            name: to_name(name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct Employees {
    index: HashMap<String, Employee>,
}

impl Employees {
    pub fn new() -> Employees {
        Employees {
            index: HashMap::new(),
        }
    }

    pub fn list(&self) -> Vec<String> {
        let mut pairs = self
            .index
            .iter()
            .map(|(key, employee)| (key, employee.name()))
            .collect::<Vec<(&String, &str)>>();
        pairs.sort_unstable_by_key(|(key, _value)| key.to_string());
        pairs
            .iter()
            .map(|(_key, name)| name.to_string())
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
}

fn to_key(value: &str) -> String {
    value.to_uppercase().to_string()
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
