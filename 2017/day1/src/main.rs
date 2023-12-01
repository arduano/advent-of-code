use shared::CollectIter;

const INPUT: &str = shared::day_input!();

fn part1() {
    let digits = INPUT.chars().map(|c| c.to_digit(10).unwrap()).to_vec();

    let mut sum = 0;
    for i in 0..digits.len() {
        let j = (i + 1) % digits.len();
        if digits[i] == digits[j] {
            sum += digits[i];
        }
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let digits = INPUT.chars().map(|c| c.to_digit(10).unwrap()).to_vec();

    let mut sum = 0;
    for i in 0..digits.len() {
        let j = (i + digits.len() / 2) % digits.len();
        if digits[i] == digits[j] {
            sum += digits[i];
        }
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
