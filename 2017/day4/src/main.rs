use shared::*;

const INPUT: &str = day_input!();

fn part1() {
    let lines = parse_lines_words(INPUT);

    let count = lines.iter().count_by(|l| !l.iter().has_duplicates());

    println!("Part 1: {}", count)
}

fn part2() {
    let lines = parse_lines_words(INPUT);

    let count = lines
        .iter()
        .count_by(|l| !l.iter().has_duplicates_by(|w| w.sort_chars()));

    println!("Part 2: {}", count)
}

fn main() {
    part1();
    part2();
}
