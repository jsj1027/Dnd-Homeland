use crate::sql_structs::class::{Class, CLASSES};
use rusqlite::{Connection, Result};
use std::fmt;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
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
                Ok(message) => {
                    let message = DbMessage::new(message);
                    self.parse_action(message);
                }
                Err(error) => match error {
                    TryRecvError::Empty => thread::sleep(Duration::from_secs(1)),
                    TryRecvError::Disconnected => thread::sleep(Duration::from_secs(1)),
                },
            }
        }
    }

    fn parse_action(&self, message: DbMessage) {
        match message.action.as_str() {
            "create" => self.create(message),
            _ => panic!("Action not implemented"),
        }
    }

    fn create(&self, message: DbMessage) {
        let possible = CLASSES.iter().any(|item| item == &message.verb);
        match possible {
            true => print!("create the class"),
            false => print!("don't create anything"),
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

        let mut action: String = "action".to_string();
        let mut verb: String = "verb".to_string();
        let mut item: Option<String> = None;

        while message_iter.peek() != None {
            let part = message_iter.next().unwrap().to_lowercase();
            let answer = check_message_part(part.as_str());
            match answer {
                Some("action") => action = part.to_string(),
                Some("verb") => verb = part.to_string(),
                Some("item") => item = Some(part.to_string()),
                None => item = None,
                Some(_) => panic!("Unusable option"),
            }
        }

        DbMessage { action, verb, item }
    }
}

impl fmt::Display for DbMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let item = self.item.clone();
        match item {
            Some(_) => write!(f, "{} {} {}", self.action, self.verb, item.unwrap()),
            None => write!(f, "{} {} ", self.action, self.verb),
        }
    }
}

pub fn check_message_part(part: &str) -> std::option::Option<&str> {
    let actions = vec!["create"];
    let verbs = vec!["bard"];
    let items = vec!["item"];

    if actions.iter().any(|item| item == &part) {
        Some("action")
    } else if verbs.iter().any(|item| item == &part) {
        Some("verb")
    } else if items.iter().any(|item| item == &part) {
        Some("item")
    } else {
        None
    }
}
