// use crate::data_connection::DatabaseConnection;

// pub(crate) trait SqlStructure {
//     pub fn new() -> Self {
//         let data_base = DatabaseConnection::new();
//         let name = get_struct_name();
//         let mut sql_query = String::from("SELECT * FROM Class WHERE name=") + name;
//         let statement = data_base.connection.prepare(&sql_query[..]).unwrap();
//         let row = statement.query(NO_PARAMS).unwrap();
        // let primary_stat: String = rows.next().unwrap().unwrap().get_unwrap(0);
        
    // }

    // fn new_random() -> Self;

    // pub fn get_struct_name() -> String;

    // fn add_string_to_query_end(query: &str, string: &str) -> String {
    //    let mut query = String::from_str(query);
    //    query = query + string;
    //    query
    // }
        
    // fn get_new_attribute(attribute: str) -> <T>;
// }