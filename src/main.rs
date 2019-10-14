// use dnd::run;
use pubsub::PubSub;
use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};
// use time::Timespec;
// // TODO clearly map out project mod structture. Map every needed mod to a part of the DND 5e character sheet. No need to be hassling with this over and over again. get it done

#[derive(Debug)]
struct ClanName {
    name: String,
    race: String,
}

fn main() -> Result<()> {
    // run();
    let pubsub = PubSub::new(1);
    let dbpath = "./data/dnd.db";
    let conn = Connection::open(&dbpath)?;
    println!("{}", conn.is_autocommit());
    // match conn.execute("SELECT * FROM Race",NO_PARAMS) {
    //     Ok(answer) => answer,
    //     Err(_) => 1,
    // };
    let mut statment = conn.prepare("SELECT * FROM 'Clan Names'").unwrap();
    let mut prep = statment.query_map(NO_PARAMS, |row| Ok(ClanName {name: row.get(0)?, race: row.get(1)?}))?;
    for item in prep {
        println!("Found Clan Name {:#?}",  item.unwrap());
    }
    Ok(())
}
