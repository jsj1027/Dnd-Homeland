use crate::data_connection::DatabaseConnection;
use rand::{thread_rng, Rng};
use rusqlite::{Statement, NO_PARAMS};

#[derive(Debug)]
pub struct Class {
    pub name: String,
    // primary: String,
    // secondary: String,
    // alternative: String,
    // cooperative: String,
}

impl Class {
    pub fn new() -> Class {
        let data_base = DatabaseConnection::new();
        let name = get_name(&data_base);
        Class { name: name }
    }
}

fn get_name(data_base: &DatabaseConnection) -> String {
    let query = String::from("SELECT name FROM Class");
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let rows = statement.query_map(NO_PARAMS, |row| row.get(0)).unwrap();
    let mut results: Vec<String> = Vec::new();
    for row in rows {
        results.push(row.unwrap());
    }
    let mut rng = thread_rng();
    let index = rng.gen_range(0, results.len());
    String::from(&results[index])
}
