pub mod sql_structs;
use sql_structs::class;
use sql_structs::data_connection::DatabaseConnection;
use std::sync::mpsc::{Sender};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_database_connection(message_channel: Sender<String>) -> (DatabaseConnection, Sender<String>) {
    DatabaseConnection::new(message_channel)
}
