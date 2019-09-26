use rand::prelude::{thread_rng, Rng, ThreadRng};

#[derive(Debug)]
pub struct Status {
    strength: [i32; 2],
    dexterity: [i32; 2],
    constitution: [i32; 2],
    intelligence: [i32; 2],
    wisdom: [i32; 2],
    charisma: [i32; 2],
}

impl Status {
    pub fn new() -> Status {
        Status {
            strength: stat_formula(0),
            dexterity: stat_formula(0),
            constitution: stat_formula(0),
            intelligence: stat_formula(0),
            wisdom: stat_formula(0),
            charisma: stat_formula(0),
        }
    }

    pub fn get_random_stats(&mut self) {
        let mut random_stats: Vec<i32> = vec![];
        let mut index = 0;
        while index < 6 {
            // 6 is the amount of stats in the Status struct
            let amount_of_dice: i32 = 4;
            let number_of_sides: i32 = 6;
            let dice_rolls: Vec<i32> = dice_roll(amount_of_dice, number_of_sides);
            let mut dice_rolls: Vec<i32> = sort(dice_rolls);
            // removes the lowest value from the sorted vector.
            dice_rolls.remove(0);
            let stat_to_add: i32 = dice_rolls.iter().sum();
            random_stats.push(stat_to_add);
            index += 1;
        }

        self.strength = stat_formula(random_stats[0]);
        self.dexterity = stat_formula(random_stats[1]);
        self.constitution = stat_formula(random_stats[2]);
        self.intelligence = stat_formula(random_stats[3]);
        self.wisdom = stat_formula(random_stats[4]);
        self.charisma = stat_formula(random_stats[5]);
    }

    pub fn get_default_stats(&mut self) {
        let mut stat_order: Vec<i32> = vec![];
        let mut rng: ThreadRng = thread_rng();
        let mut default_stat_num = vec![15, 14, 13, 12, 10, 8];
        while default_stat_num.len() > 0 {
            let num = rng.gen_range(0, default_stat_num.len());
            stat_order.push(default_stat_num[num]);
            default_stat_num.remove(num);
        }
        self.strength = stat_formula(stat_order[0]);
        self.dexterity = stat_formula(stat_order[1]);
        self.constitution = stat_formula(stat_order[2]);
        self.intelligence = stat_formula(stat_order[3]);
        self.wisdom = stat_formula(stat_order[4]);
        self.charisma = stat_formula(stat_order[5]);
    }

    pub fn increase_stat_value(&mut self, stat: &str, amount_to_increase_by: i32) {
        match stat {
            "Strength" => self.strength = stat_formula(self.strength[0] + amount_to_increase_by),
            "Dexterity" => self.dexterity = stat_formula(self.dexterity[0] + amount_to_increase_by),
            "Constitution" => {
                self.constitution = stat_formula(self.constitution[0] + amount_to_increase_by)
            }
            "Intelligence" => {
                self.intelligence = stat_formula(self.intelligence[0] + amount_to_increase_by)
            }
            "Wisdom" => self.wisdom = stat_formula(self.wisdom[0] + amount_to_increase_by),
            "Charisma" => self.charisma = stat_formula(self.charisma[0] + amount_to_increase_by),
            _ => eprintln!("The given stat is not one you can modify."),
        }
    }
}

pub fn stat_formula(stat: i32) -> [i32; 2] {
    [stat, (stat - 10) / 2]
}

fn dice_roll(amount_of_dice: i32, type_of_dice: i32) -> Vec<i32> {
    let mut rng: ThreadRng = thread_rng();
    let mut rolls: Vec<i32> = Vec::with_capacity(amount_of_dice as usize);

    for _number in 0..amount_of_dice as isize {
        rolls.push(rng.gen_range(1, type_of_dice) as i32);
    }
    rolls
}

fn sort(unsorted_vector: Vec<i32>) -> Vec<i32> {
    let mut sorted_vector = unsorted_vector;
    let vector_size: usize = sorted_vector.len();
    for index in 0..vector_size {
        let value_holder = sorted_vector[index as usize];
        let mut second_index = index;

        while second_index > 0 && value_holder < sorted_vector[second_index as usize - 1] {
            sorted_vector[second_index as usize] = sorted_vector[second_index as usize - 1];
            second_index = second_index - 1;
        }
        sorted_vector[second_index as usize] = value_holder;
    }
    sorted_vector
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_status() {
        let status: Status = Status::new();
        assert_eq!(status.strength, [0, -5]);
        assert_eq!(status.dexterity, [0, -5]);
        assert_eq!(status.constitution, [0, -5]);
        assert_eq!(status.intelligence, [0, -5]);
        assert_eq!(status.wisdom, [0, -5]);
        assert_eq!(status.charisma, [0, -5]);
    }

    #[test]
    fn test_increase_strength_stat() {
        let mut status: Status = Status::new();
        status.get_default_stats();

        let initial_strength: [i32; 2] = status.strength;

        status.increase_stat_value("Strength", 5);

        let updated_strength: [i32; 2] = status.strength;

        assert_that(&updated_strength[0]).is_equal_to(&initial_strength[0] + 5);
        assert_that(&updated_strength[1]).is_equal_to((&updated_strength[0] - 10) / 2);
    }

    #[test]
    fn test_increase_dexterity_stat() {
        let mut status: Status = Status::new();
        status.get_default_stats();

        let initial_dexterity: [i32; 2] = status.dexterity;

        status.increase_stat_value("Dexterity", 10);

        let updated_dexterity: [i32; 2] = status.dexterity;

        assert_that(&updated_dexterity[0]).is_equal_to(&initial_dexterity[0] + 10);
        assert_that(&updated_dexterity[1]).is_equal_to((&updated_dexterity[0] - 10) / 2);
    }

    #[test]
    fn test_increase_constitution_stat() {
        let mut status: Status = Status::new();
        status.get_default_stats();

        let initial_constitution: [i32; 2] = status.constitution;

        status.increase_stat_value("Constitution", -3);

        let updated_constitution: [i32; 2] = status.constitution;

        assert_that(&updated_constitution[0]).is_equal_to(&initial_constitution[0] - 3);
        assert_that(&updated_constitution[1]).is_equal_to((&updated_constitution[0] - 10) / 2);
    }

    #[test]
    fn test_increase_intelligence_stat() {
        let mut status: Status = Status::new();
        status.get_default_stats();

        let initial_intelligence: [i32; 2] = status.intelligence;

        status.increase_stat_value("Intelligence", -5);

        let updated_intelligence: [i32; 2] = status.intelligence;

        assert_that(&updated_intelligence[0]).is_equal_to(&initial_intelligence[0] - 5);
        assert_that(&updated_intelligence[1]).is_equal_to((&updated_intelligence[0] - 10) / 2);
    }

    #[test]
    fn test_increase_wisdom_stat() {
        let mut status: Status = Status::new();
        status.get_default_stats();

        let initial_wisdom: [i32; 2] = status.wisdom;

        status.increase_stat_value("Wisdom", -0);

        let updated_wisdom: [i32; 2] = status.wisdom;

        assert_that(&updated_wisdom[0]).is_equal_to(&initial_wisdom[0] + -0);
        assert_that(&updated_wisdom[1]).is_equal_to((&updated_wisdom[0] - 10) / 2);
    }

    #[test]
    fn test_increase_charisma_stat() {
        let mut status: Status = Status::new();
        status.get_default_stats();

        let initial_charisma: [i32; 2] = status.charisma;

        status.increase_stat_value("Charisma", 0);

        let updated_charisma: [i32; 2] = status.charisma;

        assert_that(&updated_charisma[0]).is_equal_to(&initial_charisma[0] + 0);
        assert_that(&updated_charisma[1]).is_equal_to((&updated_charisma[0] - 10) / 2);
    }

    #[test]
    fn test_stat_formula() {
        let stat_num = 21;
        let formula_result: [i32; 2] = stat_formula(stat_num);
        //stat_formula() rounds down and not up
        assert_eq!(formula_result, [stat_num, 5]);
        assert_ne!(formula_result, [stat_num, 6]);
    }

    #[test]
    fn test_diceroll() {
        let number_of_dice = 4;
        let number_of_sides = 4;
        let result: Vec<i32> = dice_roll(number_of_dice, number_of_sides);
        assert_that(&result).has_length(number_of_dice as usize);
        for dice_result in &result {
            assert_that(dice_result).is_less_than_or_equal_to(number_of_sides);
            assert_that(dice_result).is_greater_than(0);
        }
    }
    #[test]
    fn test_sort() {
        let unsorted_vector: Vec<i32> = vec![23, 6, 35, 2];
        let unsorted_vector_second: Vec<i32> = vec![35, 23, 6, 2];
        let init_sorted: Vec<i32> = vec![2, 6, 23, 35];
        let sorted_vector: Vec<i32> = sort(unsorted_vector);
        let sorted_vector_second: Vec<i32> = sort(unsorted_vector_second);
        for x in 0..sorted_vector.len() {
            assert_eq!(init_sorted[x], sorted_vector[x]);
            assert_eq!(init_sorted[x], sorted_vector_second[x]);
        }
    }
}
