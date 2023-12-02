use shared::*;

const INPUT: &str = day_input!();

#[derive(Debug, Default)]
struct GameCubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    number: u32,
    cubes: Vec<GameCubes>,
}

fn max_gamecubes(left: &GameCubes, right: &GameCubes) -> GameCubes {
    GameCubes {
        red: left.red.max(right.red),
        green: left.green.max(right.green),
        blue: left.blue.max(right.blue),
    }
}

fn parse_line(line: &str) -> Game {
    let mut parts = line.split(": ");
    let number = parts
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let cubes = parts
        .next()
        .unwrap()
        .split("; ")
        .map(|cubes| {
            let mut parts = cubes.split(", ");
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            while let Some(part) = parts.next() {
                let mut part = part.split(" ");
                let count = part.next().unwrap().parse::<u32>().unwrap();
                let color = part.next().unwrap();
                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => panic!("Unknown color: {}", color),
                }
            }

            GameCubes { red, green, blue }
        })
        .collect();
    Game { number, cubes }
}

fn part1() {
    let games = INPUT.lines().map(parse_line).collect::<Vec<_>>();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut total_possible_sum = 0;
    for game in games {
        let mut possible = true;
        for cubes in game.cubes {
            if cubes.red > max_red || cubes.green > max_green || cubes.blue > max_blue {
                possible = false;
                break;
            }
        }

        if possible {
            total_possible_sum += game.number;
        }
    }

    println!("Part 1: {}", total_possible_sum)
}

fn part2() {
    let games = INPUT.lines().map(parse_line).collect::<Vec<_>>();

    let mut sum = 0;

    for game in games {
        let mut max = Default::default();
        for cubes in game.cubes {
            max = max_gamecubes(&max, &cubes);
        }

        sum += max.red * max.green * max.blue;
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
