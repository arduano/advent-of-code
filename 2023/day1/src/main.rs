use shared::*;

const INPUT: &str = day_input!();

fn char_as_number(c: &char) -> Option<u32> {
    c.to_digit(10)
}

fn part1() {
    let lines = parse_lines::<String>(INPUT);

    let mut sum = 0;

    for line in lines {
        let chars = line.chars().collect::<Vec<_>>();

        let first_digit = chars.iter().find_map(char_as_number);
        let last_digit = chars.iter().rev().find_map(char_as_number);

        let number = first_digit.unwrap() * 10 + last_digit.unwrap();

        sum += number;
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let lines = parse_lines::<String>(INPUT);

    let digit_words = [
        ("zero", 0),
        ("0", 0),
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    let mut sum = 0;
    for line in lines {
        let mut number = 0;
        for i in 0..line.len() {
            let line = &line[i..];

            let digit = digit_words.iter().find_map(|(word, digit)| {
                if line.starts_with(word) {
                    Some(digit)
                } else {
                    None
                }
            });

            if let Some(digit) = digit {
                number = digit * 10;
                break;
            }
        }

        for i in (0..=line.len()).rev() {
            let line = &line[..i];

            let digit = digit_words.iter().find_map(|(word, digit)| {
                if line.ends_with(word) {
                    Some(digit)
                } else {
                    None
                }
            });

            if let Some(digit) = digit {
                number += digit;
                break;
            }
        }

        sum += number;
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
