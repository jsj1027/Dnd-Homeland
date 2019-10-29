mod class;
mod data_connection;
mod race;
mod status;

use class::Class;
use race::Race;
use status::Status;

fn main() {
    let rand_race = Race::new();
    println!("Race Name: {:#?}", rand_race.name);
    println!("Speed: {:#?}", rand_race.speed);
    println!("Age: {:#?}", rand_race.age);
    println!("Languages: {:#?}", rand_race.languages);
    println!("Proficienes: {:#?}", rand_race.proficienes);
    println!("Sizes: {:#?}, {:.1?}", rand_race.size.0, rand_race.size.1);
    println!("Bonus Stat: {:#?}\n\n", rand_race.stat_bonus);
    let mut new_status = Status::new(true);
    println!(
        "strength score:{:#?}, modifer:{:#?}",
        new_status.strength.score, new_status.strength.modifer
    );
    println!(
        "dexterity score:{:#?}, modifer:{:#?}",
        new_status.dexterity.score, new_status.dexterity.modifer
    );
    println!(
        "constitution score:{:#?}, modifer:{:#?}",
        new_status.constitution.score, new_status.constitution.modifer
    );
    println!(
        "intelligence score:{:#?}, modifer:{:#?}",
        new_status.intelligence.score, new_status.intelligence.modifer
    );
    println!(
        "wisdom score:{:#?}, modifer:{:#?}",
        new_status.wisdom.score, new_status.wisdom.modifer
    );
    println!(
        "charisma score:{:#?}, charisma:{:#?}",
        new_status.charisma.score, new_status.charisma.modifer
    );
    println!(
        "\n\nInital strength score:{:#?}, modifer:{:#?}",
        new_status.strength.score, new_status.strength.modifer
    );
    new_status.strength.change_by(&3);
    println!(
        "Modded strength score:{:#?}, modifer:{:#?}",
        new_status.strength.score, new_status.strength.modifer
    );
    new_status.strength.set_to(&20);
    println!(
        "Set to strength score:{:#?}, modifer:{:#?}",
        new_status.strength.score, new_status.strength.modifer
    );

    let rand_class = Class::new();
    println!("Class name: {:#?}", rand_class.name);
    println!("Class primary stat: {:#?}", rand_class.primary_stat);
    println!("Class secondary stat: {:#?}", rand_class.secondary_stat);
}
