mod data_connection;
mod race;
use race::Race;

fn main() {
    let rand_race = Race::new();
    println!("{:#?}", rand_race.name);
}
// data_loader::connect(&pubber);
// pubber.notify("select", "SELECT * FROM 'Clan Names'");

// let dbpath = "./data/dnd.db";
// let conn = Connection::open(&dbpath)?;
// println!("{}", conn.is_autocommit());
// match conn.execute("SELECT * FROM Race",NO_PARAMS) {
//     Ok(answer) => answer,
//     Err(_) => 1,
// };
// let mut statment = conn.prepare("SELECT * FROM 'Clan Names'").unwrap();
// let prep = statment.query_map(NO_PARAMS, |row| Ok(ClanName {name: row.get(0)?, race: row.get(1)?}))?;
// for item in prep {
//     println!("Found Clan Name {:#?}",  item.unwrap());
// }
// Ok(())
