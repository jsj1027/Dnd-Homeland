use rand::{thread_rng, Rng};
use rusqlite::{NO_PARAMS, Statement};
use crate::data_connection::DatabaseConnection;

// #[derive(Debug)]
// enum Size {
//     Tiny,
//     Small,
//     Medium,
// }

#[derive(Debug)]
pub struct Race {
    pub name: String,
    // age: i32,
    // size: (Size, String),
    // speed: i32,
    // languages: Vec<String>,
    // proficienes: Vec<Vec<String>>,
}

impl Race {
    pub fn new() -> Race {
        Race {
            name: get_name(),
        }
            // age: 20,
            // size: self.determine_size(name, full_race_data),
            // speed: 30,
            // languages: vec![String::from("Common"), String::from("Elvish")],
            // proficienes: vec![
            //     vec![String::from("Deception"), String::from("History")],
            //     vec![String::from("TOols?"), String::from("Tools?")],
            // ],
        
    }

}
    fn get_name() -> String{
        let data_base =  DatabaseConnection::new();
        let query = String::from("SELECT Name FROM Race");
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
