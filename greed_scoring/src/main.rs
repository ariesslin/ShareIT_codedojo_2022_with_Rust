#![allow(dead_code)]
use std::collections::HashMap;
static SET_SIZE: i32 = 3;
struct DiceContainer {
    dice_classifier: HashMap<i32, i32>,
}

impl DiceContainer {
    fn new() -> DiceContainer {
        Self {
            dice_classifier: HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]),
        }
    }

    fn reset_new_game(&mut self) {
        self.dice_classifier = HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    }

    fn dice_classify_processor(&mut self, dices: Vec<i32>) {
        for dice in dices {
            *self.dice_classifier.get_mut(&dice).unwrap() += 1;
        }
    }

    fn apply_a_set_of_three_ones_is_1000_points(&self) -> i32 {
        1000 * (self.dice_classifier.get(&1).unwrap() / SET_SIZE)
    }

    fn apply_a_set_of_three_numbers_other_than_ones_is_worth_100_times_the_number(&self) -> i32 {
        let mut score = 0;
        for num in 2..6 + 1 {
            score += 100 * num * (self.dice_classifier.get(&num).unwrap() / SET_SIZE)
        }
        score
    }

    fn apply_a_one_that_is_not_part_of_a_set_of_three_is_worth_100_points(&self) -> i32 {
        100 * (self.dice_classifier.get(&1).unwrap() % SET_SIZE)
    }

    fn apply_a_file_that_is_not_part_of_a_set_of_three_is_worth_50_points(&self) -> i32 {
        50 * (self.dice_classifier.get(&5).unwrap() % SET_SIZE)
    }

    pub fn get_greed_score(&mut self, dices: Vec<i32>) -> i32 {
        self.reset_new_game();
        self.dice_classify_processor(dices);
        let mut score = 0;

        score += self.apply_a_set_of_three_ones_is_1000_points();
        score += self.apply_a_set_of_three_numbers_other_than_ones_is_worth_100_times_the_number();
        score += self.apply_a_one_that_is_not_part_of_a_set_of_three_is_worth_100_points();
        score += self.apply_a_file_that_is_not_part_of_a_set_of_three_is_worth_50_points();

        score
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_greed_score_when_given_the_dice_sequence() {
        let mut dice_container = DiceContainer::new();

        let dices = vec![1, 1, 1, 5, 1];
        let result = dice_container.get_greed_score(dices);
        assert_eq!(result, 1150);

        let dices = vec![2, 3, 4, 6, 2];
        let result = dice_container.get_greed_score(dices);
        assert_eq!(result, 0);

        let dices = vec![3, 4, 5, 3, 3];
        let result = dice_container.get_greed_score(dices);
        assert_eq!(result, 350);

        let dices = vec![1, 5, 1, 2, 4];
        let result = dice_container.get_greed_score(dices);
        assert_eq!(result, 250);

        let dices = vec![1, 2, 2, 2, 3];
        let result = dice_container.get_greed_score(dices);
        assert_eq!(result, 300);
    }
}
