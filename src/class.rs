use crate::data_connection::DatabaseConnection;
use rand::{thread_rng, Rng};
use rusqlite::{Row, Statement, NO_PARAMS};


trait SqlStructure {
    fn get_struct_name() -> String;

    fn new() {
        let data_base = DatabaseConnection::new();
        let name = SqlStructure::get_struct_name();
        let mut sql_query = String::from("SELECT * FROM Class WHERE name=") + &name;
        let statement = data_base.connection.prepare(&sql_query[..]).unwrap();
        let row = statement.query(NO_PARAMS).unwrap();
        // let primary_stat: String = rows.next().unwrap().unwrap().get_unwrap(0);
        
    }

    // fn new_random() -> Self;

    // fn add_string_to_query_end(query: &str, string: &str) -> String {
    //    let mut query = String::from_str(query);
    //    query = query + string;
    //    query
    // }
        
    // fn get_new_attribute(attribute: str) -> <T>;
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub primary_stat: String,
    pub secondary_stat: String,
    // pub alternative_stat: String,
    pub cooperative_stat: String,
}

impl Class {
    pub fn new() {

        // let data_base = DatabaseConnection::new();
        // let name = get_name(&data_base);
        // let formatted_name = get_formatted_name(&name);
        // Class {
        //     primary_stat: get_primary_stat(&formatted_name, &data_base),
        //     secondary_stat: get_secondary_stat(&formatted_name, &data_base),
        //     cooperative_stat: get_cooperative_stat(&formatted_name, &data_base),
        //     name: name,
        // }
    }
}

// fn get_name(data_base: &DatabaseConnection) -> String {
//     let query = String::from("SELECT name FROM Class");
//     let mut statement: Statement = data_base.connection.prepare(&query[..]).unwrap();
//     let rows = statement.query_map(NO_PARAMS, |row| row.get(0)).unwrap();
//     let mut results: Vec<String> = Vec::new();
//     for row in rows {
//         results.push(row.unwrap());
//     }
//     let mut rng = thread_rng();
//     let index = rng.gen_range(0, results.len());
//     String::from(&results[index])
// }

// fn get_formatted_name(unformatted_name: &str) -> String {
//     let mut base = String::from("''");
//     base.insert_str(1, unformatted_name);
//     base
// }

// fn get_primary_stat(race_name: &String, data_base: &DatabaseConnection) -> String {
//     let mut query: String = String::from("SELECT primaryStat FROM Class WHERE name=");
//     query.push_str(&race_name);
//     let mut statement = data_base.connection.prepare(&query[..]).unwrap();
//     let mut rows = statement.query(NO_PARAMS).unwrap();
//     let primary_stat: String = rows.next().unwrap().unwrap().get_unwrap(0);
//     primary_stat
// }

// fn get_secondary_stat(race_name: &String, data_base: &DatabaseConnection) -> String {
//     let mut query: String = String::from("SELECT secondaryStat FROM Class WHERE name=");
//     query.push_str(&race_name);
//     let mut statement = data_base.connection.prepare(&query[..]).unwrap();
//     let mut rows = statement.query(NO_PARAMS).unwrap();
//     let secondary_stat: String = rows.next().unwrap().unwrap().get_unwrap(0);
//     secondary_stat
// }

// fn get_cooperative_stat(race_name: &String, data_base: &DatabaseConnection) -> String{
//     let mut query: String = String::from("SELECT cooperativeStat FROM Class WHERE name=");
//     query.push_str(&race_name);
//     let mut statement = data_base.connection.prepare(&query[..]).unwrap();
//     let mut rows = statement.query(NO_PARAMS).unwrap();
//     let cooperative_stat = rows.next().unwrap().unwrap().get_unwrap(0);
//     cooperative_stat
// }
