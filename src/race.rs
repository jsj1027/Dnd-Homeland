use crate::data_connection::DatabaseConnection;
use rand::{thread_rng, Rng};
use rusqlite::{Statement, NO_PARAMS};

#[derive(Debug)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
pub struct Race {
    pub name: String,
    pub age: i32,
    pub size: (Size, f64),
    pub speed: i32,
    pub languages: Vec<String>,
    pub proficienes: Vec<String>,
}

impl Race {
    pub fn new() -> Race {
        let data_base = DatabaseConnection::new();
        let name = get_name(&data_base);
        let formatted_name = get_formatted_name(&name);
        Race {
            speed: get_speed(&formatted_name, &data_base),
            age: get_age(&formatted_name, &data_base),
            languages: get_language(&formatted_name, &data_base),
            proficienes: get_proficienes(&formatted_name, &data_base),
            size: get_size(&formatted_name, &data_base),
            name: name,
        }
        // size: self.determine_size(name, full_race_data),
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

fn get_formatted_name(unformatted_name: &str) -> String {
    let mut base = String::from("''");
    base.insert_str(1, unformatted_name);
    base
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
    vec![String::from("Common"), languages]
}

fn get_proficienes(race_name: &str, data_base: &DatabaseConnection) -> Vec<String> {
    let mut query: String = String::from("SELECT weaponProf, toolProf FROM Race WHERE name=");
    query.push_str(&race_name);
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let row = rows.next().unwrap().unwrap();
    let weapon_prof: String = row.get_unwrap(0);
    let tool_prof: String = row.get_unwrap(1);
    vec!(weapon_prof, tool_prof)
}

fn get_size(race_name: &str, data_base: &DatabaseConnection) -> (Size, f64) {
    let mut query: String = String::from("SELECT minSize, maxSize FROM Race WHERE name=");
    query.push_str(&race_name);
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let row = rows.next().unwrap().unwrap();

    let min_size: f64 = row.get_unwrap(0) ;
    let min_size: f64 = (min_size* 10.0).round()/ 10.0;
    let max_size: f64 = row.get_unwrap(1);
    let max_size: f64 = (max_size* 10.0).round()/ 10.0;

    let mut rng = thread_rng();
    let size_number: f64 = rng.gen_range(min_size, max_size);
    
    const TINY_START: f64 = 1.0;
    const TINY_END: f64 = 1.9;
    const SMALL_START: f64 = 2.0;
    const SMALL_END: f64 = 3.9;
    const MEDIUM_START: f64 = 4.0;
    const MEDIUM_END: f64 = 7.9;
    const LARGE_START: f64 = 8.0;
    const LARGE_END: f64 = 15.9;


    let size_type : Size = match size_number.round() {
        TINY_START ..= TINY_END => Size::Tiny,
        SMALL_START ..= SMALL_END => Size::Small,
        MEDIUM_START ..= MEDIUM_END => Size::Medium,
        LARGE_START ..= LARGE_END => Size::Large,
        _ => panic!("Size not within allowed range.")
    };
    
    (size_type, size_number)
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
