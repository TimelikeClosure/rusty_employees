use super::super::errors::QueryError;
use super::employees::Employees;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub struct Department {
    name: String,
    employees: Employees,
}

impl Department {
    pub fn new(name: &String) -> Department {
        Department {
            name: to_name(name),
            employees: Employees::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn employees(&mut self) -> &mut Employees {
        &mut self.employees
    }

    pub fn assign(&mut self, employee_name: &str) -> Result<String, QueryError> {
        self.employees().create(employee_name)
    }
}

pub struct Departments {
    index: BTreeMap<String, Department>,
}

impl Departments {
    pub fn new() -> Departments {
        Departments {
            index: BTreeMap::new(),
        }
    }

    pub fn department(&mut self, department_name: &str) -> Result<&mut Department, QueryError> {
        match self.index.get_mut(&to_key(department_name)) {
            None => Err(QueryError::NotFound(format!(
                "Department \"{}\" does not exist",
                department_name
            ))),
            Some(department) => Ok(department),
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.index
            .iter()
            .map(|(_key, department)| department.name().to_string())
            .collect::<Vec<String>>()
    }

    pub fn create(&mut self, department: &String) -> Result<String, QueryError> {
        match self.index.entry(to_key(department)) {
            Entry::Vacant(entry) => {
                entry.insert(Department::new(department));
                Ok(to_name(department))
            }
            Entry::Occupied(_) => Err(QueryError::Conflict(format!(
                "Department \"{}\" already exists",
                department
            ))),
        }
    }

    pub fn delete(&mut self, department: &String) -> Result<(), QueryError> {
        match self.index.remove(&to_key(department)) {
            None => Err(QueryError::NotFound(format!(
                "Department \"{}\" not found",
                department
            ))),
            Some(_) => Ok(()),
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
