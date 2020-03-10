mod departments;
mod dummy_data;
mod employees;
use super::errors::QueryError;
use departments::{Department, Departments};

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Store {
    index: Departments,
}

impl Store {
    pub fn new() -> Self {
        Store {
            index: Departments::new(),
        }
    }

    pub fn seed(&mut self) {
        dummy_data::populate(self);
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

#[cfg(test)]
mod tests {
    use super::*;

    mod store {
        use super::*;

        mod seed {
            use super::Store;

            #[test]
            fn populates_store() {
                let mut store = Store::new();

                store.seed();

                assert_ne!(0, store.departments().list().len());
            }
        }

        mod departments {
            use super::{Departments, Store};

            #[test]
            fn returns_departments() {
                let store = Store::new();

                assert_eq!(&(Departments::new()), store.departments());
            }
        }

        mod departments_mut {
            use super::{Departments, Store};

            #[test]
            fn returns_departments() {
                let mut store = Store::new();

                assert_eq!(&mut (Departments::new()), store.departments_mut());
            }
        }

        mod department {
            use super::{Department, QueryError, Store};

            #[test]
            fn department_exists() {
                let mut store = Store::new();

                store.departments_mut().create("Assets").unwrap();

                assert_eq!(Ok(&(Department::new("Assets"))), store.department("Assets"));
            }

            #[test]
            fn department_doesnt_exist() {
                let store = Store::new();

                assert_eq!(
                    Err(QueryError::NotFound(
                        "Department \"Twinkies\" not found".to_string()
                    )),
                    store.department("Twinkies")
                );
            }
        }

        mod department_mut {
            use super::{Department, QueryError, Store};

            #[test]
            fn department_exists() {
                let mut store = Store::new();

                store.departments_mut().create("Liabilities").unwrap();

                assert_eq!(
                    Ok(&mut (Department::new("Liabilities"))),
                    store.department_mut("Liabilities")
                );
            }

            #[test]
            fn department_doesnt_exist() {
                let mut store = Store::new();

                assert_eq!(
                    Err(QueryError::NotFound(
                        "Department \"Donuts\" not found".to_string()
                    )),
                    store.department_mut("Donuts")
                );
            }
        }
    }
}
