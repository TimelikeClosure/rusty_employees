mod departments;
mod employees;
use departments::Departments;

pub struct Store {
    index: Departments,
}

impl Store {
    pub fn new() -> Store {
        Store {
            index: Departments::new(),
        }
    }

    pub fn departments(&mut self) -> &mut Departments {
        &mut self.index
    }
}
