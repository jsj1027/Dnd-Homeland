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
    pub stat_bonus: String,
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
            stat_bonus: get_stat_bonus(&formatted_name, &data_base),
            name: name,
        }
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
    println!("{:#?}", query);
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
    vec![weapon_prof, tool_prof]
}

fn get_size(race_name: &str, data_base: &DatabaseConnection) -> (Size, f64) {
    let mut query: String = String::from("SELECT minSize, maxSize FROM Race WHERE name=");
    query.push_str(&race_name);
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let row = rows.next().unwrap().unwrap();

    let min_size: f64 = row.get_unwrap(0);
    let max_size: f64 = row.get_unwrap(1);

    let mut rng = thread_rng();
    let size_number: f64 = rng.gen_range(min_size, max_size);

    let tiny_start: f64 = 1.0;
    let tiny_end: f64 = 1.9;
    let small_start: f64 = 2.0;
    let small_end: f64 = 3.9;
    let medium_start: f64 = 4.0;
    let medium_end: f64 = 7.9;
    let large_start: f64 = 8.0;
    let large_end: f64 = 15.9;

    if size_number >= tiny_start && size_number <= tiny_end {
        (Size::Tiny, size_number)
    } else if size_number >= small_start && size_number <= small_end {
        (Size::Small, size_number)
    } else if size_number >= medium_start && size_number <= medium_end {
        (Size::Medium, size_number)
    } else if size_number >= large_start && size_number <= large_end {
        (Size::Large, size_number)
    } else {
        (Size::Large, size_number)
    }
}

fn get_stat_bonus(race_name: &str, data_base: &DatabaseConnection) -> String {
    let mut query: String = String::from("SELECT statBonus FROM Race WHERE name=");
    query.push_str(&race_name);
    let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let stats: String = rows.next().unwrap().unwrap().get_unwrap(0);
    stats
}
