use pubsub::PubSub;
use rusqlite::{Connection, Result, Rows, Statement, NO_PARAMS};

#[derive(Debug)]
pub struct DatabaseConnection {
    path: String,
    pub connection: Connection,
}

impl DatabaseConnection {
    pub fn new(path: String) -> Self {
        DatabaseConnection {
            connection: connect(&path).unwrap(),
            path: path,
        }
    }

    pub fn return_information(self, query: String) -> Result<Vec<String>> {
        let mut statement: Statement = self.connection.prepare(&query[..]).unwrap();
        let mut rows = statement.query_map(NO_PARAMS, |row| row.get(0))?;
        let mut results = Vec::new();
        for row in rows{
            results.push(row?);
        }
        Ok(results)
    }
}

fn connect(path: &String) -> Result<Connection> {
    match Connection::open(path) {
        Ok(connection) => Ok(connection),
        Err(error) => {
            eprintln!("Could not establish connection: {}", error);
            Err(error)
        }
    }
}
// pub fn connect(ps: &PubSub) {
//     let dbpath = "./data/dnd.db";
//     let conn = Connection::open(&dbpath).unwrap();
//     println!("Hello");

//     let sub1 = ps.subscribe("select", move |query: String| {
//         let mut state = conn.prepare(&query[..]).unwrap();
//         let prep = state.query_map(NO_PARAMS, |row| Ok(ClanName {name: row.get(0).unwrap(), race: row.get(1).unwrap()})).unwrap();
//         for item in prep {
//             println!("Found Clan Name {:#?}",  item.unwrap());
//         }
//     });
// }
