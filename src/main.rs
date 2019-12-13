mod class;
mod data_connection;

use class::Class;
use class::SqlStructure;

fn main() {
    let rand_class = Class::new("'Barbarian'");
    let rand_class = Class::random_new();
}
