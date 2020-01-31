use super::super::errors::QueryError;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Departments {
    index: HashMap<String, String>,
}

impl Departments {
    pub fn new() -> Departments {
        Departments {
            index: vec![
                String::from("Accounting"),
                String::from("Design"),
                String::from("Engineering"),
                String::from("Logistics"),
                String::from("Production"),
                String::from("Purchasing"),
                String::from("Sales"),
            ]
            .iter()
            .fold(HashMap::new(), |mut departments, department| {
                departments.insert(to_key(&department), to_name(&department));
                departments
            }),
        }
    }

    pub fn list(&self) -> Vec<String> {
        let mut pairs = self.index.iter().collect::<Vec<(&String, &String)>>();
        pairs.sort_unstable_by_key(|(key, _value)| key.to_string());
        pairs
            .iter()
            .map(|(_key, name)| name.to_string())
            .collect::<Vec<String>>()
    }

    pub fn create(&mut self, department: &String) -> Result<String, QueryError> {
        match self.index.entry(to_key(department)) {
            Entry::Vacant(entry) => {
                entry.insert(to_name(department));
                Ok(to_name(department))
            }
            Entry::Occupied(_) => Err(QueryError::Conflict(format!(
                "Department \"{}\" already exists",
                department
            ))),
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
