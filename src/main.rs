// use dnd::run;
use pubsub::PubSub;
use rusqlite::{Connection, Result};
// // TODO clearly map out project mod structture. Map every needed mod to a part of the DND 5e character sheet. No need to be hassling with this over and over again. get it done

fn main() {
    // run();
    let pubsub = PubSub::new(1);
    let dbpath = "../data/dnd.db";
    let conn = Connection::open(&dbpath);
    let conn = match conn {
            Ok(connection) => connection,
            Err(_) => panic!("Could not connect to database.")   
        };
    println!("{}", conn.is_autocommit());
}
