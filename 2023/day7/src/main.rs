use std::collections::HashMap;

use shared::*;

const INPUT: &str = day_input!();

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
struct Card(char);

impl Card {
    pub fn as_number(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => self.0.to_digit(10).unwrap(),
        }
    }

    pub fn as_number_2(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 0, // Joker
            'T' => 10,
            _ => self.0.to_digit(10).unwrap(),
        }
    }
}

enum HandKind {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, PartialEq)]
struct Hand {
    // Always 5 cards
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn as_count_map(&self) -> HashMap<Card, u32> {
        let mut map = HashMap::new();
        for &card in &self.cards {
            *map.entry(card).or_insert(0) += 1;
        }
        map
    }

    fn hand_kind(&self) -> HandKind {
        let count_map = self.as_count_map();

        let mut vals = count_map.values().cloned().collect::<Vec<_>>();
        vals.sort();
        vals.reverse();

        if vals[0] == 5 {
            return HandKind::FiveOfAKind;
        }

        if vals[0] == 4 {
            return HandKind::FourOfAKind;
        }

        if vals[0] == 3 && vals[1] == 2 {
            return HandKind::FullHouse;
        }

        if vals[0] == 3 {
            return HandKind::ThreeOfAKind;
        }

        if vals[0] == 2 && vals[1] == 2 {
            return HandKind::TwoPairs;
        }

        if vals[0] == 2 {
            return HandKind::OnePair;
        }

        HandKind::HighCard
    }

    fn hand_kind_2(&self) -> HandKind {
        let mut count_map = self.as_count_map();
        let joker_count = count_map.remove(&Card('J')).unwrap_or(0);

        let mut vals = count_map.values().cloned().collect::<Vec<_>>();
        vals.sort();
        vals.reverse();

        if joker_count == 5 {
            return HandKind::FiveOfAKind;
        }

        if vals[0] + joker_count == 5 {
            return HandKind::FiveOfAKind;
        }

        if vals[0] + joker_count == 4 {
            return HandKind::FourOfAKind;
        }

        if vals[0] + joker_count >= 3
            && vals[1] + joker_count >= 2
            && vals[0] + vals[1] + joker_count == 5
        {
            return HandKind::FullHouse;
        }

        if vals[0] + joker_count >= 3 {
            return HandKind::ThreeOfAKind;
        }

        if vals[0] + joker_count >= 2
            && vals[1] + joker_count >= 2
            && vals[0] + vals[1] + joker_count == 4
        {
            return HandKind::TwoPairs;
        }

        if vals[0] + joker_count >= 2 {
            return HandKind::OnePair;
        }

        HandKind::HighCard
    }

    fn is_cards_stronger_than(&self, other: &Self) -> bool {
        // Compare card numbers
        for (mine, other) in self.cards.iter().zip(other.cards.iter()) {
            if mine.as_number() > other.as_number() {
                return true;
            } else if mine.as_number() < other.as_number() {
                return false;
            }
        }

        // Default value, never reached in input
        true
    }

    fn is_hand_stronger_than(&self, other: &Self) -> bool {
        let my_kind = self.hand_kind() as u32;
        let other_kind = other.hand_kind() as u32;

        if my_kind > other_kind {
            return true;
        } else if my_kind < other_kind {
            return false;
        }

        // Same kind, compare cards
        self.is_cards_stronger_than(other)
    }

    fn is_cards_stronger_than_2(&self, other: &Self) -> bool {
        // Compare card numbers
        for (mine, other) in self.cards.iter().zip(other.cards.iter()) {
            if mine.as_number_2() > other.as_number_2() {
                return true;
            } else if mine.as_number_2() < other.as_number_2() {
                return false;
            }
        }

        // Default value, never reached in input
        true
    }

    fn is_hand_stronger_than_2(&self, other: &Self) -> bool {
        let my_kind = self.hand_kind_2() as u32;
        let other_kind = other.hand_kind_2() as u32;

        if my_kind > other_kind {
            return true;
        } else if my_kind < other_kind {
            return false;
        }

        // Same kind, compare cards
        self.is_cards_stronger_than_2(other)
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_hand_stronger_than(other) {
            Some(std::cmp::Ordering::Greater)
        } else if other.is_hand_stronger_than(self) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

fn parse_input() -> Vec<Hand> {
    INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let code = parts.next().unwrap();
            let number = parts.next().unwrap().parse().unwrap();
            Hand {
                cards: code.chars().map(Card).collect(),
                bid: number,
            }
        })
        .collect()
}

fn part1() {
    let hands = parse_input();

    // Sort
    let mut hands = hands;
    hands.sort_by(|a, b| {
        if a.is_hand_stronger_than(b) {
            std::cmp::Ordering::Greater
        } else if b.is_hand_stronger_than(a) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        let bid = hand.bid;
        let bid = bid * (i + 1) as u32;
        sum += bid;
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let hands = parse_input();

    // Sort
    let mut hands = hands;
    hands.sort_by(|a, b| {
        if a.is_hand_stronger_than_2(b) {
            std::cmp::Ordering::Greater
        } else if b.is_hand_stronger_than_2(a) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        let bid = hand.bid;
        let bid = bid * (i + 1) as u32;
        sum += bid;
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
