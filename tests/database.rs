use employees::database::{Database, QueryResponse, Table};

#[test]
fn user_can_exit() {
    let mut db = Database::new();

    assert_eq!(QueryResponse::Exit, db.query("exit".to_string()));
}
