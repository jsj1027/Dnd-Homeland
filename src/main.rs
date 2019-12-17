mod class;
mod data_connection;
mod traits;

use class::Class;
use rusqlite::{Connection, Error};
// use class::SqlStructure;

fn main() {
    // let rand_class = Class::new("'Barbarian'");
    // let rand_class = Class::random_new();
    let db_connection: Result<Connection, Error> =  Connection::open("./data/dnd.db");
    
}
