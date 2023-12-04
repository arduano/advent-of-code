use std::collections::HashSet;

use shared::*;

const INPUT: &str = day_input!();

#[derive(Debug)]
struct Card {
    _id: u32,
    first: Vec<u32>,
    second: Vec<u32>,
}

fn parse_card(str: &str) -> Card {
    let mut parts = str.split(": ");

    let id = parts
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut parts = parts.next().unwrap().split(" | ");
    let first = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let second = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    Card {
        _id: id,
        first,
        second,
    }
}

fn parse_input() -> Vec<Card> {
    let lines = parse_lines::<String>(INPUT);
    lines.into_iter().map(|s| parse_card(&s)).collect()
}

fn get_card_matches(card: &Card) -> u32 {
    let all_winning_numbers = card.first.iter().collect::<HashSet<_>>();
    let mut matching_numbers = 0;
    for number in card.second.iter() {
        if all_winning_numbers.contains(number) {
            matching_numbers += 1;
        }
    }

    matching_numbers
}

fn part1() {
    let cards = parse_input();

    let mut sum = 0;
    for card in cards {
        let all_winning_numbers = card.first.iter().collect::<HashSet<_>>();
        let mut matching_numbers = 0;
        for number in card.second.iter() {
            if all_winning_numbers.contains(number) {
                matching_numbers += 1;
            }
        }

        if matching_numbers >= 1 {
            sum += 1 << (matching_numbers - 1)
        }
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let cards = parse_input();

    let mut sum = 0;
    for i in 0..cards.len() {
        dbg!(i);
        let mut stack = vec![i];
        while let Some(i2) = stack.pop() {
            sum += 1;
            let card = &cards[i2];
            let matches = get_card_matches(card);

            for n in 0..matches {
                stack.push(i2 + n as usize + 1);
            }
        }
    }

    println!("Part 2: {}", sum)
}

fn part2_improved() {
    let cards = parse_input();
    let card_matches = cards
        .iter()
        .map(|card| get_card_matches(card))
        .collect::<Vec<_>>();

    let mut match_counts = vec![0; cards.len()];

    // Dynamically go from end to start, calculating
    for i in (0..cards.len()).rev() {
        let matches = card_matches[i] as usize;

        let match_counts_slice = &mut match_counts[i + 1..i + matches + 1];
        let sum = match_counts_slice.iter().sum::<usize>();

        match_counts[i] = sum + 1;
    }

    let sum = match_counts.iter().sum::<usize>();

    println!("Part 2 improved: {}", sum)
}

fn main() {
    part1();
    part2();
    part2_improved();
}
