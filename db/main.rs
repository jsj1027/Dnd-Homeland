mod data_connection;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

// use class::Class;
use rusqlite::{Connection, Error};
// use class::SqlStructure;

fn main() {
    let db_connection: Result<Connection, Error> =  Connection::open("./data/dnd.db");
    let db_connection = match  db_connection {
        Ok(connection) => connection,
        Err(_) => panic!("database error'd on open."),
    };
    
}
