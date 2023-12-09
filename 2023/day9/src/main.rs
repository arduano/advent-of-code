use shared::*;

const INPUT: &str = day_input!();

fn parse_input() -> Vec<Vec<i32>> {
    // Split by lines and then by whitespaces
    INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn derivative(arr: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..arr.len() - 1 {
        result.push(arr[i + 1] - arr[i]);
    }

    result
}

fn derivatives_until_zero(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![arr.clone()];

    let mut arr = arr;

    loop {
        let derivative = derivative(arr);

        if derivative.iter().all(|&x| x == 0) {
            break;
        }

        result.push(derivative.clone());

        arr = derivative;
    }

    result
}

fn extrapolated(arr: Vec<i32>) -> i32 {
    let derivatives = derivatives_until_zero(arr);

    let mut prev = 0;
    for i in (0..derivatives.len()).rev() {
        let derivative = &derivatives[i];
        prev += derivative.last().unwrap();
    }

    prev
}

fn extrapolated_back(arr: Vec<i32>) -> i32 {
    let derivatives = derivatives_until_zero(arr);

    let mut prev = 0;
    for i in (0..derivatives.len()).rev() {
        let derivative = &derivatives[i];
        prev = derivative.first().unwrap() - prev;
    }

    prev
}

fn part1() {
    let input = parse_input();

    // Sum all extrapolations
    let sum = input
        .iter()
        .map(|arr| extrapolated(arr.clone()))
        .sum::<i32>();

    println!("Part 1: {}", sum)
}

fn part2() {
    let input = parse_input();

    // Backwards
    let sum = input
        .iter()
        .map(|arr| extrapolated_back(arr.clone()))
        .sum::<i32>();

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
