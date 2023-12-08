use std::collections::HashMap;

use shared::*;

const INPUT: &str = day_input!();

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
struct Instructions {
    dirs: Vec<Dir>,
    parts: HashMap<String, (String, String)>,
}

fn parse_dir(c: char) -> Dir {
    match c {
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => panic!("Invalid direction"),
    }
}

fn parse_input() -> Instructions {
    let mut dirs = Vec::new();

    let lines = INPUT.lines().collect::<Vec<_>>();
    let dirs_line = lines[0];

    for c in dirs_line.chars() {
        dirs.push(parse_dir(c));
    }

    let mut parts_dict = HashMap::new();
    let mut first = None;

    // VHS = (XDL, LVL)
    // MLQ = (VXK, CJK)
    // CNX = (CTN, DRK)
    // MVD = (SRT, SRT)

    for line in &lines[2..] {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        let mut parts = value.split(", ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let left = left[1..left.len()].to_string();
        let right = right[0..right.len() - 1].to_string();

        parts_dict.insert(key.to_string(), (left, right));

        if first.is_none() {
            first = Some(key.to_string());
        }
    }

    Instructions {
        dirs,
        parts: parts_dict,
    }
}

fn part1() {
    let input = parse_input();

    let mut current = "AAA";

    let mut count = 0;
    for dir in input.dirs.iter().cycle() {
        let (left, right) = input.parts.get(current).unwrap();

        let next = match dir {
            Dir::Left => left,
            Dir::Right => right,
        };

        count += 1;

        if next == "ZZZ" {
            break;
        }

        current = next;
    }

    println!("Part 1: {}", count)
}

fn find_cycle(
    dirs: &[Dir],
    code: &String,
    parts: &HashMap<String, (String, String)>,
) -> (u64, Vec<bool>) {
    let mut current = code;

    let mut found_vals = HashMap::<(u64, &String), u64>::new();

    let mut ends_with_z_bools = Vec::new();

    let mut count = 0;
    for (i, dir) in dirs.iter().enumerate().cycle() {
        let (left, right) = parts.get(current).unwrap();

        let next = match dir {
            Dir::Left => left,
            Dir::Right => right,
        };

        count += 1;

        let key = (i as u64, next);

        if next.ends_with("Z") {
            ends_with_z_bools.push(true);
        } else {
            ends_with_z_bools.push(false);
        }

        if let Some(&start) = found_vals.get(&key) {
            // Truncate by start
            ends_with_z_bools.drain(..(start as usize));
            return (start, ends_with_z_bools);
        }

        found_vals.insert(key, count);
        current = next;
    }

    unreachable!();
}

fn find_multiples(cycle_start: u64, cycle_bools: &[bool]) -> Vec<Period> {
    let mut result = Vec::new();

    for (i, &ends_with_z) in cycle_bools.iter().enumerate() {
        if ends_with_z {
            result.push(Period {
                start_offset: cycle_start + i as u64 + 1,
                stride: cycle_bools.len() as u64,
            });
        }
    }

    result
}

#[derive(Debug, Clone, Copy)]
struct Period {
    start_offset: u64,
    stride: u64,
}

fn find_overlap_time(periods: &[Period]) -> Option<u64> {
    if periods.is_empty() {
        return None;
    }

    let max_stride = periods.iter().map(|p| p.stride).max().unwrap_or(0);

    // The time must be at least the maximum of start_offsets.
    let mut time = periods.iter().map(|p| p.start_offset).max().unwrap_or(0);

    // Keep checking until we reach the least common multiple of all strides.
    while time <= lcm_of_all_strides(periods) {
        if periods
            .iter()
            .all(|p| (time >= p.start_offset) && ((time - p.start_offset) % p.stride == 0))
        {
            return Some(time);
        }
        time += max_stride;
    }

    None
}

fn lcm_of_all_strides(periods: &[Period]) -> u64 {
    periods
        .iter()
        .map(|p| p.stride)
        .fold(1, |acc, stride| lcm(acc, stride))
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

// Every permutation with one element from each set
fn permutations<T: Copy>(numbers: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::<Vec<T>>::new();

    for i in 0..numbers.len() {
        let mut new_result = Vec::new();

        for j in 0..numbers[i].len() {
            if result.len() == 0 {
                new_result.push(vec![numbers[i][j]]);
            } else {
                for r in &result {
                    let mut new_r = r.clone();
                    new_r.push(numbers[i][j]);
                    new_result.push(new_r);
                }
            }
        }

        result = new_result;
    }

    result
}

fn part2() {
    let input = parse_input();

    let current = input
        .parts
        .keys()
        .filter(|k| k.ends_with("A"))
        .cloned()
        .to_set();

    let cycles = current
        .iter()
        .map(|s| (s, find_cycle(&input.dirs, s, &input.parts)))
        .to_vec();

    let cycles = cycles
        .iter()
        .map(|(s, (start, ends_with_z_bools))| (s, find_multiples(*start, ends_with_z_bools)))
        .to_vec();

    let permutations = permutations(cycles.iter().map(|(_, periods)| periods.clone()).collect());

    let mut min_multiple = u64::MAX;

    for permutation in permutations {
        let overlap_time = find_overlap_time(&permutation);
        if let Some(overlap_time) = overlap_time {
            if overlap_time < min_multiple {
                min_multiple = overlap_time;
            }
        }
    }

    println!("Part 2: {}", min_multiple)
}

fn main() {
    part1();
    part2();
}
