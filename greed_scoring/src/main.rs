pub fn get_greed_score(dices: Vec<i32>) -> i32 {
    let mut score = 0;
    let mut skip = 0;
    let mut dices_iter = dices.clone();
    dices_iter.sort();
    dices_iter.push(-1);
    dices_iter.push(-1);

    for seq in dices_iter.windows(3) {
        println!("the seq is {:?}", seq);
        if skip != 0 {
            skip -= 1;
            continue;
        }
        if seq == vec![1, 1, 1] {
            score += 1000;
            skip = 2;
        } else if seq[0] == seq[1] && seq[0] == seq[2] {
            score += 100 * seq[0];
            skip = 2;
        } else if seq[0] == 5 {
            score += 50;
        } else if seq[0] == 1 {
            score += 100;
        }
    }
    score
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_greed_score_when_given_the_dice_sequence() {
        let dices = vec![1, 1, 1, 5, 1];
        let result = get_greed_score(dices);
        assert_eq!(result, 1150);

        let dices = vec![2, 3, 4, 6, 2];
        let result = get_greed_score(dices);
        assert_eq!(result, 0);

        let dices = vec![3, 4, 5, 3, 3];
        let result = get_greed_score(dices);
        assert_eq!(result, 350);

        let dices = vec![1, 5, 1, 2, 4];
        let result = get_greed_score(dices);
        assert_eq!(result, 250);
    }
}