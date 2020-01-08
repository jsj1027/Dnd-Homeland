pub mod structs;
use std::sync::mpsc::Sender;
use structs::class;
use structs::data_connection::DatabaseConnection;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_database_connection(
    message_channel: Sender<String>,
) -> (DatabaseConnection, Sender<String>) {
    DatabaseConnection::new(message_channel)
}
