#![allow(dead_code)]

use derivative::Derivative;
use derive_builder::Builder;
use std::collections::HashMap;
static SET_SIZE: i32 = 3;
struct DiceContainer {
    dice_classifier: HashMap<i32, i32>,
}
#[derive(Derivative, Default, Builder, Debug)]
#[builder(setter(into), default)]
struct Rule {
    dices: Vec<i32>,
    points: i32,
    #[derivative(Default(value = "false"))]
    is_set: bool,
    #[derivative(Default(value = "false"))]
    multiple_dice: bool,
    description: String,
}

impl DiceContainer {
    fn new() -> DiceContainer {
        Self {
            dice_classifier: HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]),
        }
    }

    fn reset_new_game(&mut self) {
        self.dice_classifier = Self::new().dice_classifier;
    }

    fn dice_classify_processor(&mut self, dices: Vec<i32>) {
        for dice in dices {
            *self.dice_classifier.get_mut(&dice).unwrap() += 1;
        }
    }

    fn apply_rule(&self, rule: Rule) -> i32 {
        let mut score = 0;
        for dice in rule.dices {
            score += rule.points
                * match rule.multiple_dice {
                    true => dice,
                    false => 1,
                }
                * match rule.is_set {
                    true => self.dice_classifier.get(&dice).unwrap() / SET_SIZE,
                    false => self.dice_classifier.get(&dice).unwrap() % SET_SIZE,
                };
        }
        score
    }

    pub fn get_greed_score(&mut self, dices: Vec<i32>) -> i32 {
        self.reset_new_game();
        self.dice_classify_processor(dices);
        let mut score = 0;

        score += self.apply_rule(
            RuleBuilder::default()
                .dices(vec![1])
                .points(1000)
                .is_set(true)
                .description("A set of three ones is 1000 points")
                .build()
                .unwrap(),
        );

        score += self.apply_rule(
            RuleBuilder::default()
                .dices(vec![2, 3, 4, 5, 6])
                .points(100)
                .is_set(true)
                .multiple_dice(true)
                .description(
                    "A set of three numbers (other than ones) is worth 100 times the number",
                )
                .build()
                .unwrap(),
        );

        score += self.apply_rule(
            RuleBuilder::default()
                .dices(vec![1])
                .points(100)
                .description("A one (that is not part of a set of three) is worth 100 points")
                .build()
                .unwrap(),
        );

        score += self.apply_rule(
            RuleBuilder::default()
                .dices(vec![5])
                .points(50)
                .description("A five (that is not part of a set of three) is worth 50 points")
                .build()
                .unwrap(),
        );

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
