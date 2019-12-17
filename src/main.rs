#[macro_use]
extern crate clap;
use clap::App;
use std::thread;
fn main() {
    let yaml = load_yaml!("configuration.yaml");
    let matches = App::from(yaml).get_matches();
    
    let character = match matches.value_of("new") {
        Some(character) => character,
        _ => return,
    };
    match character {
        "bard" | "Bard"| "BARD" => println!("we gonna create a bard"),
        _ => println!("Not a creatable class."),
    }
}