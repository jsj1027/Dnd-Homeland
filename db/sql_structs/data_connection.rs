use rusqlite::{Connection, Result};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct DatabaseConnection {
    pub connection: Connection,
    pub output_channel: Sender<String>,
    pub intake_channel: Receiver<String>,
}

impl DatabaseConnection {
    pub fn new(output_channel: Sender<String>) -> (Self, Sender<String>) {
        let path = "./data/dnd.db";
        let (db_output_channel, db_intake_channel): (Sender<String>, Receiver<String>) = mpsc::channel();
        let connection = DatabaseConnection {
            connection: connect(&path).unwrap(),
            output_channel,
            intake_channel: db_intake_channel,
        };
        (connection, db_output_channel)
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
