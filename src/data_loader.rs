use pubsub::PubSub;
use rusqlite::{Connection, Result, NO_PARAMS};

// static pubsub: PubSub = PubSub::new(1);
#[derive(Debug)]
struct ClanName {
    name: String,
    race: String,
}


pub fn connect(ps: &PubSub) {
    let dbpath = "./data/dnd.db";
    let conn = Connection::open(&dbpath).unwrap();
    println!("Hello");

    let sub1 = ps.subscribe("select", move |query: String| {
        let mut state = conn.prepare(&query[..]).unwrap();
        let prep = state.query_map(NO_PARAMS, |row| Ok(ClanName {name: row.get(0).unwrap(), race: row.get(1).unwrap()})).unwrap();
        for item in prep {
            println!("Found Clan Name {:#?}",  item.unwrap());
        }
    });
}
// use serde_yaml::Mapping;
// use serde_json::{Map, Value};
// use std::fs::File;
// use std::io::{Error, ErrorKind};

// #[derive(Debug)]
// pub struct ProjectConfiguration {
//     config: Mapping,
// }

// impl ProjectConfiguration {
//     pub fn read() -> ProjectConfiguration {
//         let file: Result<File, Error> = File::open("config/configuration.yaml");
//         let file: File = match file {
//             Ok(file) => file,
//             Err(error) => match error.kind() {
//                 ErrorKind::NotFound => {
//                     panic!("configuration.yaml not found: {:#?}", ErrorKind::NotFound)
//                 }
//                 other_error => panic!(
//                     "File was found but something went wrong opening it: {:#?}",
//                     other_error
//                 ),
//             },
//         };
//         let yaml: Mapping = serde_yaml::from_reader(file).expect("File should be proper yaml");
//         ProjectConfiguration { config: yaml }
//     }

//     pub fn return_config(&self) -> &Mapping {
//         &self.config
//     }
// }

// #[derive(Debug)]
// pub struct JsonData {
//     data: Map<String, Value>,
// }

// impl JsonData {
//     pub fn new(file_path: String) -> JsonData {
//         let file: Result<File, Error> = File::open(file_path);
//         let file: File = match file {
//             Ok(file) => file,
//             Err(error) => match error.kind() {
//                 ErrorKind::NotFound => panic!("race.json not found: {:#?}", ErrorKind::NotFound),
//                 other_error => panic!(
//                     "Race.json was found but something went wrong opening it: {:#?}",
//                     other_error
//                 ),
//             },
//         };
//         let json: serde_json::Value =
//             serde_json::from_reader(file).expect("race.json is not proper json");
//         let json: Map<String, Value> = json.as_object().unwrap().clone();
//         JsonData { data: json }
//     }

//     pub fn get_data(&self) -> &Map<String, Value> {
//         &self.data
//     }
// }
