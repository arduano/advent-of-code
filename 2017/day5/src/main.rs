use shared::*;

const INPUT: &str = day_input!();

fn part1() {
    let mut numbers = parse_lines::<i32>(INPUT);

    let mut index = 0i32;
    let mut count = 0;

    while index >= 0 && index < numbers.len() as i32 {
        let offset = numbers[index as usize];
        numbers[index as usize] += 1;
        index += offset;
        count += 1;
    }

    println!("Part 1: {}", count)
}

fn part2() {
    let mut numbers = parse_lines::<i32>(INPUT);

    let mut index = 0i32;
    let mut count = 0;

    while index >= 0 && index < numbers.len() as i32 {
        let offset = numbers[index as usize];

        if offset >= 3 {
            numbers[index as usize] -= 1;
        } else {
            numbers[index as usize] += 1;
        }

        index += offset;
        count += 1;
    }

    println!("Part 2: {}", count)
}

fn main() {
    part1();
    part2();
}
