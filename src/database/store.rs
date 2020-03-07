mod departments;
mod dummy_data;
mod employees;
use super::errors::QueryError;
use departments::{Department, Departments};

pub struct Store {
    index: Departments,
}

impl Store {
    pub fn new() -> Store {
        let mut store = Store {
            index: Departments::new(),
        };
        dummy_data::populate(&mut store);
        store
    }

    pub fn departments(&self) -> &Departments {
        &self.index
    }

    pub fn departments_mut(&mut self) -> &mut Departments {
        &mut self.index
    }

    pub fn department(&self, department_name: &str) -> Result<&Department, QueryError> {
        self.index.department(department_name)
    }

    pub fn department_mut(&mut self, department_name: &str) -> Result<&mut Department, QueryError> {
        self.index.department_mut(department_name)
    }
}
