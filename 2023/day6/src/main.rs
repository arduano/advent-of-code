use shared::*;

const INPUT: &str = day_input!();

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

// Input:
// ```
// Time:      7  15   30
// Distance:  9  40  200
// ```

fn parse_input() -> Vec<Race> {
    let lines = INPUT.lines().to_vec();
    let time_line = lines[0];
    let distance_line = lines[1];

    let times = time_line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .to_vec();

    let distances = distance_line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .to_vec();

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .to_vec()
}

fn is_winning_condition(race: Race, held_time: u64) -> bool {
    let remaining_time = race.time - held_time;
    let speed_gained = held_time;
    let distance = remaining_time * speed_gained;

    return distance > race.distance;
}

fn win_condition_count_brute_force(race: Race) -> u64 {
    let press_range = 0..race.time;

    press_range
        .filter(move |held_time| is_winning_condition(race, *held_time))
        .count() as u64
}

fn part1() {
    let input = parse_input();

    let mut mult = 1;
    for race in input {
        dbg!(race);
        dbg!(win_condition_count_brute_force(race));
        mult *= win_condition_count_brute_force(race);
    }

    println!("Part 1: {}", mult)
}

fn parse_input2() -> Race {
    let lines = INPUT.lines().to_vec();
    let time_line = lines[0];
    let distance_line = lines[1];

    let time = time_line
        .split_whitespace()
        .skip(1)
        .to_vec()
        .join("")
        .parse()
        .unwrap();
    let distance = distance_line
        .split_whitespace()
        .skip(1)
        .to_vec()
        .join("")
        .parse()
        .unwrap();

    return Race {
        time: time,
        distance: distance,
    };
}

fn part2() {
    let input = parse_input2();

    let a = -1f64;
    let b = input.time as f64;
    let c = -(input.distance as f64);

    let left = (-b + (b.powi(2) - 4f64 * a * c).sqrt()) / (2f64 * a);
    let right = (-b - (b.powi(2) - 4f64 * a * c).sqrt()) / (2f64 * a);

    let left = left as u64;
    let right = right as u64;

    let tries_left = (left - 2)..=(left + 2);
    let tries_right = (right - 2)..=(right + 2);

    let mut min_success = u64::MAX;
    for i in tries_left {
        if is_winning_condition(input, i) {
            min_success = min_success.min(i);
        }
    }

    let mut max_success = 0;
    for i in tries_right {
        if is_winning_condition(input, i) {
            max_success = max_success.max(i);
        }
    }

    let range = max_success - min_success + 1;

    println!("Part 2: {}", range);
}

fn main() {
    part1();
    part2();
}
