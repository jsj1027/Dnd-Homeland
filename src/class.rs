use crate::data_connection::DatabaseConnection;
use rusqlite::{NO_PARAMS};


pub trait SqlStructure {
    fn get_struct_name() -> String;

    fn new() {
        let data_base = DatabaseConnection::new();
        let name = SqlStructure::get_struct_name();
        let mut sql_query = String::from("SELECT * FROM Class WHERE name=") + &name;
        let statement = data_base.connection.prepare(&sql_query[..]).unwrap();
        let row = statement.query(NO_PARAMS).unwrap();
        // let primary_stat: String = rows.next().unwrap().unwrap().get_unwrap(0);
        
    }
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub primary_stat: String,
    pub secondary_stat: String,
    pub alternative_stat: String,
    pub cooperative_stat: String,
}

impl Class {
    pub fn new() {

    }
}