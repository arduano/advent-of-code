use shared::*;

const INPUT: &str = day_input!();

fn part1() {
    let data = parse_lines_items_int(INPUT);

    let sum = data.iter().map(|l| l.max_val() - l.min_val()).sum2();

    println!("Part 1: {}", sum)
}

fn part2() {
    let data = parse_lines_items_int(INPUT);

    let sum = data
        .iter()
        .map(|l| {
            l.into_iter()
                .get_matching_pair_values(|&a, &b| a % b == 0)
                .into_iter()
                .map(|(a, b)| a / b)
                .sum2()
        })
        .sum2();

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
