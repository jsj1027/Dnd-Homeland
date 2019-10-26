mod data_connection;
mod race;
use race::Race;

fn main() {
    let rand_race = Race::new();
    println!("Race Name: {:#?}", rand_race.name);
    println!("Speed: {:#?}", rand_race.speed);
    println!("Age: {:#?}", rand_race.age);
    println!("Languages: {:#?}", rand_race.languages);
    println!("Proficienes: {:#?}", rand_race.proficienes);
    println!("Sizes: {:#?}, {:.1?}", rand_race.size.0, rand_race.size.1);
    println!("Bonus Stat: {:#?}", rand_race.stat_bonus);
}
