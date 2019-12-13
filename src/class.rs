use crate::data_connection::DatabaseConnection;
use rusqlite::{Row,Error, NO_PARAMS};
use std::collections::HashMap;
pub trait SqlStructure {
    fn get_struct_name() -> String;


    fn new(class_name: &str)  {
        let data_base = DatabaseConnection::new();
        let struct_name = <Class as SqlStructure>::get_struct_name();
        let sql_query = String::from("SELECT * FROM ") + &struct_name +" WHERE name=" + &class_name;
        let mut statement = data_base.connection.prepare(&sql_query[..]).unwrap();
        let mut row = statement.query(NO_PARAMS).unwrap();
        let thing = row.next().unwrap().unwrap();
        let primary_stat: String = thing.get_unwrap(0);
        let s_stat: String = thing.get_unwrap(1);
        let a_stat: String = thing.get_unwrap(2);
        let c_stat: String = thing.get_unwrap(3);
        println!("primstat {:#?}", primary_stat);
        println!("primstat {:#?}", s_stat);
        println!("primstat {:#?}", a_stat);
        println!("primstat {:#?}", c_stat);
    }

    fn random_new() {
        let data_base = DatabaseConnection::new();
        let struct_name = <Class as SqlStructure>::get_struct_name();
        let sql_query = String::from("SELECT * FROM ") + &struct_name +" ORDER BY RANDOM() LIMIT 1";
        let mut statement = data_base.connection.prepare(&sql_query[..]).unwrap();
        let mut row = statement.query(NO_PARAMS).unwrap();
        let primary_stat: String = row.next().unwrap().unwrap().get_unwrap(0);
        println!("primstat {:#?}", primary_stat);
    }
}


#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub primary_stat: String,
    pub secondary_stat: String,
    pub alternative_stat: String,
    pub cooperative_stat: String,
}

impl SqlStructure for Class {
    fn get_struct_name() -> String {
        String::from("Class")
    }
}
