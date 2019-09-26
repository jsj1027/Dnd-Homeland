#[macro_use]
extern crate serde_json;
use serde_yaml::Value;

mod class;
mod data_loader;
mod profile;
mod race;
mod status;

// use class::ClassList;
use data_loader::ProjectConfiguration;
use profile::Profile;
// use status::Status;

pub fn get_randomized_status() -> status::Status {
    let mut status = status::Status::new();
    status.get_random_stats();
    status
}

pub fn get_defaulted_status() -> status::Status {
    let mut status = status::Status::new();
    status.get_default_stats();
    return status;
}

pub fn get_race() -> race::Race {
    let race_file = data_loader::JsonData::new(String::from("data/race.json"));
    let race_file_data = race_file.get_data();
    let race = race::Race::new(String::from("Human"), race_file_data);
    race
}

pub fn run() {
    let profile: Profile = Profile::new();
    println!("Profile: {:#?}", profile);
    let config_data = ProjectConfiguration::read();
    let yaml_config = config_data.return_config();
    let yaml_data = yaml_config.get(&serde_yaml::Value::from("profile")).unwrap();
    println!("Config: {:#?}", yaml_data["name"]);
    let race_file: data_loader::JsonData = data_loader::JsonData::new(String::from("data/race.json"));
    let mut new_race = get_race();
    println!("size: {:#?}", new_race);
}
