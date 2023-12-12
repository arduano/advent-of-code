use std::collections::HashMap;

use shared::*;

const INPUT: &str = day_input!();

// ???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Operational, // .
    Damaged,     // #
    Unknown,     // ?
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Item::Operational => '.',
            Item::Damaged => '#',
            Item::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

fn items_match(a: Item, b: Item) -> bool {
    match (a, b) {
        (Item::Operational, Item::Operational) => true,
        (Item::Damaged, Item::Damaged) => true,
        (Item::Unknown, _) | (_, Item::Unknown) => true,
        _ => false,
    }
}

fn item_from_char(c: char) -> Item {
    match c {
        '.' => Item::Operational,
        '#' => Item::Damaged,
        '?' => Item::Unknown,
        _ => panic!("Invalid item char: {}", c),
    }
}

#[derive(Debug)]
struct Row {
    items: Vec<Item>,
    spans: Vec<usize>,
}

fn parse_line(line: &str) -> Row {
    // Split by space, get items then spans
    let mut parts = line.split(' ');
    let items = parts.next().unwrap().chars().map(item_from_char).collect();
    let spans = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Row { items, spans }
}

fn parse_input() -> Vec<Row> {
    INPUT.lines().map(parse_line).collect()
}

fn matches_span_len_from(items: &[Item], len: usize, from: usize) -> bool {
    if len == 0 || from + len > items.len() {
        return false;
    }

    if from > 0 {
        if !items_match(items[from - 1], Item::Operational) {
            return false;
        }
    }

    if from + len < items.len() {
        if !items_match(items[from + len], Item::Operational) {
            return false;
        }
    }

    for i in from..from + len {
        if !items_match(items[i], Item::Damaged) {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RecursiveState {
    items_pos: usize,
    remaining_spans_len: usize,
}

fn recursively_fit(
    items: &[Item],
    items_pos: usize,
    remaining_spans: &[usize],
    cache: &mut HashMap<RecursiveState, u64>,
) -> u64 {
    let state = RecursiveState {
        items_pos,
        remaining_spans_len: remaining_spans.len(),
    };

    if let Some(&count) = cache.get(&state) {
        return count;
    }

    if remaining_spans.len() == 0 {
        // Check that all remaining items are operational
        for i in items_pos..items.len() {
            if !items_match(items[i], Item::Operational) {
                cache.insert(state, 0);
                return 0;
            }
        }

        cache.insert(state, 1);
        return 1;
    }

    let next_span = remaining_spans[0];
    let mut sum = 0;

    for i in items_pos..items.len() {
        if matches_span_len_from(items, next_span, i) {
            sum += recursively_fit(items, i + next_span + 1, &remaining_spans[1..], cache);
        }

        if items[i] == Item::Damaged {
            break;
        }
    }

    cache.insert(state, sum);
    sum
}

fn part1() {
    let input = parse_input();

    let mut sum = 0;

    for row in input {
        let mut cache = HashMap::new();
        let count = recursively_fit(&row.items, 0, &row.spans, &mut cache);
        sum += count;
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let input = parse_input();

    let mut sum = 0;

    for mut row in input {
        let items = row.items.clone();
        row.items.push(Item::Unknown);
        row.items.extend(&items);
        row.items.push(Item::Unknown);
        row.items.extend(&items);
        row.items.push(Item::Unknown);
        row.items.extend(&items);
        row.items.push(Item::Unknown);
        row.items.extend(&items);

        let spans = row.spans.clone();
        row.spans.extend(&spans);
        row.spans.extend(&spans);
        row.spans.extend(&spans);
        row.spans.extend(&spans);

        let mut cache = HashMap::new();
        let count = recursively_fit(&row.items, 0, &row.spans, &mut cache);
        sum += count;
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
