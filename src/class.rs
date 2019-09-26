use rand::prelude::*;
use serde_json::{Map, Value};
use std::fs::File;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Class {
    name: String,
    primary: String,
    secondary: String,
    alternative: String,
    cooperative: String,
}

#[derive(Debug)]
pub struct ClassList {
    data: Map<String, Value>,
}

impl ClassList {
    pub fn new() -> ClassList {
        let file: Result<File, Error> = File::open("data/class.json");
        let file: File = match file {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("class.json not found: {:#?}", ErrorKind::NotFound),
                other_error => panic!(
                    "class.json was found but something went wrong opening it: {:#?}",
                    other_error
                ),
            },
        };
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("class.json is not proper json");
        let json: Map<String, Value> = json.as_object().unwrap().clone();
        ClassList { data: json }
    }

    pub fn get_random(&self) -> Class {
        let class_names: Vec<_> = self.data.keys().collect();
        let mut rng: ThreadRng = thread_rng();
        let random_value: usize = rng.gen_range(0, class_names.len());
        let class_name = class_names[random_value].to_string();
        let found_class = json!(&self.data[&class_name]);

        Class {
            name: class_name,
            primary: found_class["primary"].to_string(),
            secondary: found_class["secondary"].to_string(),
            alternative: found_class["alternative"].to_string(),
            cooperative: found_class["cooperative"].to_string(),
        }
    }
}
