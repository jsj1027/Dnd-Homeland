use crate::data_connection::DatabaseConnection;
use rand::{thread_rng, Rng};
use rusqlite::{Row, Statement, NO_PARAMS};

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub primary_stat: String,
    pub secondary_stat: String,
    // pub alternative_stat: String,
    pub cooperative_stat: String,
}

impl Class {
    pub fn new() -> Class {
        let data_base = DatabaseConnection::new();
        let name = get_name(&data_base);
        let formatted_name = get_formatted_name(&name);
        Class {
            primary_stat: get_primary_stat(&formatted_name, &data_base),
            secondary_stat: get_secondary_stat(&formatted_name, &data_base),
            cooperative_stat: get_cooperative_stat(&formatted_name, &data_base),
            name: name,
        }
    }
}

fn get_name(data_base: &DatabaseConnection) -> String {
    let query = String::from("SELECT name FROM Class");
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

fn get_primary_stat(race_name: &String, data_base: &DatabaseConnection) -> String {
    let mut query: String = String::from("SELECT primaryStat FROM Class WHERE name=");
    query.push_str(&race_name);
    let mut statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let primary_stat: String = rows.next().unwrap().unwrap().get_unwrap(0);
    primary_stat
}

fn get_secondary_stat(race_name: &String, data_base: &DatabaseConnection) -> String {
    let mut query: String = String::from("SELECT secondaryStat FROM Class WHERE name=");
    query.push_str(&race_name);
    let mut statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let secondary_stat: String = rows.next().unwrap().unwrap().get_unwrap(0);
    secondary_stat
}

fn get_cooperative_stat(race_name: &String, data_base: &DatabaseConnection) -> String{
    let mut query: String = String::from("SELECT cooperativeStat FROM Class WHERE name=");
    query.push_str(&race_name);
    let mut statement = data_base.connection.prepare(&query[..]).unwrap();
    let mut rows = statement.query(NO_PARAMS).unwrap();
    let cooperative_stat = rows.next().unwrap().unwrap().get_unwrap(0);
    cooperative_stat
}
