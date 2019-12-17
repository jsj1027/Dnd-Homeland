use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct DatabaseConnection {
    path: String,
    pub connection: Connection,
}

impl DatabaseConnection {
    pub fn new() -> Self {
        let path = "./data/dnd.db";
        DatabaseConnection {
            connection: connect(&path).unwrap(),
            path: String::from(path),
        }
    }
}

fn connect(path: &str) -> Result<Connection> {
    match Connection::open(path) {
        Ok(connection) => Ok(connection),
        Err(error) => {
            eprintln!("Could not establish connection: {}", error);
            Err(error)
        }
    }
}