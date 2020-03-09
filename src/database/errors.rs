#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum QueryError {
    Conflict(String),
    NotFound(String),
}
