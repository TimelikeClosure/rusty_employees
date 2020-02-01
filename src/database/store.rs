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

    pub fn departments(&mut self) -> &mut Departments {
        &mut self.index
    }

    pub fn department(&mut self, department_name: &str) -> Result<&mut Department, QueryError> {
        self.index.department(department_name)
    }
}
