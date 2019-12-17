use rusqlite::{Connection, Result};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct DatabaseConnection {
    pub connection: Connection,
    pub output_channel: Sender<String>,
    pub intake_channel: Receiver<String>,
}

impl DatabaseConnection {
    pub fn new(output_channel: Sender<String>) -> (Self, Sender<String>) {
        let path = "./data/dnd.db";
        let (db_output_channel, db_intake_channel): (Sender<String>, Receiver<String>) =
            mpsc::channel();
        let connection = DatabaseConnection {
            connection: connect(&path).unwrap(),
            output_channel,
            intake_channel: db_intake_channel,
        };
        (connection, db_output_channel)
    }

    pub fn run(&self) {
        let check = true;
        while check {
            match self.intake_channel.try_recv() {
                Ok(message) => println!("this is the message: {}", message),
                Err(error) => match error {
                    TryRecvError::Empty => thread::sleep(Duration::from_secs(1)),
                    TryRecvError::Disconnected => thread::sleep(Duration::from_secs(1)),
                },
            }
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

#[derive(Debug, Clone)]
pub struct DbMessage {
    action: String,
    verb: String,
    item: Option<String>,
}

impl DbMessage {
    fn new(message: String) -> Self {
        let mut message_iter = message.split('_').peekable();

        let action: String;
        let verb: String;
        let item: Option<String>;

        while message_iter.peek() != None {
            let part = message_iter.next();
            let check_part = check_message_part(String::from(part.unwrap()));
        }

        DbMessage {
            action: String::from("Hi"),
            verb: String::from("Hi"),
            item: Some(String::from("Hi")),
        }
    }
}

pub fn check_message_part(part: String) -> (std::option::Option<std::string::String>, std::option::Option<std::string::String>) {
    let part = part.to_lowercase();
    
    let actions = vec![String::from("create")];
    let verbs = vec![String::from("bard")];
    
    if actions.iter().any(|item| item == &part) {
        (Some("action".to_string()), Some(part))
    }
    else if verbs.iter().any(|item| item == &part) {
        (Some("verb".to_string()), Some(part))
    }
    else {
        (None, None)
    }
} 
