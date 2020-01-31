pub mod departments;
use departments::Departments;

pub struct Store {
    pub departments: Departments,
}

impl Store {
    pub fn new() -> Store {
        Store {
            departments: Departments::new(),
        }
    }
}
