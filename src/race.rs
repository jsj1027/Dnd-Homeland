use crate::data_connection::DatabaseConnection;
use rand::{thread_rng, Rng};
use rusqlite::{Statement, NO_PARAMS};

// #[derive(Debug)]
// enum Size {
//     Tiny,
//     Small,
//     Medium,
// }

#[derive(Debug)]
pub struct Race {
    pub name: String,
    pub age: i32,
    // size: (Size, String),
    pub speed: i32,
    pub languages: Vec<String>,
    // proficienes: Vec<Vec<String>>,
}

impl Race {
    pub fn new() -> Race {
        let data_base = DatabaseConnection::new();
        let name =  get_name(&data_base);
        let formatted_name = get_formatted_name(&name);
        Race {
            speed: get_speed(&formatted_name, &data_base),
            age: get_age(&formatted_name, &data_base),
            languages: get_language(&formatted_name, &data_base),
            name: name,
        }
        // age: 20,
        // size: self.determine_size(name, full_race_data),
        // languages: vec![String::from("Common"), String::from("Elvish")],
        // proficienes: vec![
        //     vec![String::from("Deception"), String::from("History")],
        //     vec![String::from("TOols?"), String::from("Tools?")],
        // ],
    }
}

fn get_name(data_base: &DatabaseConnection) -> String {
    let query = String::from("SELECT name FROM Race");
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let rows = statement.query_map(NO_PARAMS, |row| row.get(0)).unwrap();
    let mut results: Vec<String> = Vec::new();
    for row in rows {
        results.push(row.unwrap());
    }
    let mut rng = thread_rng();
    let index = rng.gen_range(0, results.len());
    String::from(&results[index])
}

fn get_speed(race_name: &String, data_base: &DatabaseConnection) -> i32 {
    let mut query: String = String::from("SELECT speed FROM Race WHERE name=");
    query.push_str(&race_name);
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let speed: i32 = rows.next().unwrap().unwrap().get_unwrap(0);
    speed
}

fn get_age(race_name: &String, data_base: &DatabaseConnection) -> i32 {
    let mut query: String = String::from("SELECT minAge, maxAge FROM Race WHERE name=");
    query.push_str(&race_name);
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let row = rows.next().unwrap().unwrap();
    let min_age: i32 = row.get_unwrap(0);
    let max_age: i32 = row.get_unwrap(1);
    let mut rng = thread_rng();
    let age = rng.gen_range(min_age, max_age);
    age
}

fn get_language(race_name: &str, data_base: &DatabaseConnection) -> Vec<String> {
    let mut query: String = String::from("SELECT languages FROM Race WHERE name=");
    query.push_str(&race_name);
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let languages: String = rows.next().unwrap().unwrap().get_unwrap(0);
    vec!(String::from("Common"), languages)
}

fn get_formatted_name(unformatted_name: &str) -> String {
    let mut base = String::from("''");
    base.insert_str(1, unformatted_name);
    base
}
// //     fn determine_size(&self, race: String, full_race_data: &serde_json::Map<String, serde_json::Value>) {
// //         let current_race_data = &full_race_data[&race];
// //         let max_size: &f64 = &current_race_data["maxSize"].as_f64().unwrap();
// //         let min_size: &f64 = &current_race_data["minSize"].as_f64().unwrap();
// //         let height = rand::thread_rng().gen_range(min_size, max_size);
// //         let height: f64 = (height * 100.0).round() / 100.0;
// //         println!("{}", height);
// //         if height >= 1.0 && height < 2.0 {
// //             self.size = (Size::Tiny, format!("{:.2} feet and inches", height))
// //         } else if height >= 2.0 && height < 4.0 {
// //             self.size = (Size::Small, format!("{:.2} feet and inches", height))
// //         } else if height >= 4.0 && height < 8.0 {
// //             self.size = (Size::Medium, format!("{:.2} feet and inches", height))
// //         } else {
// //             panic!(
// //                 "Race size is either too big or too small. Size determined is {}",
// //                 height
// //             )
// //         }
// // }
// }
