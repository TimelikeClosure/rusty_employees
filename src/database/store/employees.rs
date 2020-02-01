use super::super::errors::QueryError;
use super::departments::Departments;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Employee {
    name: String,
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
}

fn to_key(value: &str) -> String {
    value.to_uppercase().to_string()
}

fn to_name(value: &str) -> String {
    value
        .chars()
        .enumerate()
        .map(|(index, character)| {
            if index == 0 {
                character.to_uppercase().next().unwrap()
            } else {
                character.to_lowercase().next().unwrap()
            }
        })
        .collect::<String>()
}
