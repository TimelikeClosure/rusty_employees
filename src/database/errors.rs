#[derive(Debug)]
pub enum QueryError {
    Conflict(String),
    NotFound(String),
}
