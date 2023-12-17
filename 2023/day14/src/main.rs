use std::str::FromStr;

use shared::*;

const INPUT: &str = day_input!();

// O....#....
// O.OO#....#
// .....##...
// OO.#O....O
// .O.....O#.
// O.#..O.#.#
// ..O..#O..O
// .......O..
// #....###..
// #OO..#....

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    RollingRock, // O
    StaticRock,  // #
    Empty,       // .
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(cell_from_char(s.chars().next().unwrap()))
    }
}

fn cell_from_char(c: char) -> Cell {
    match c {
        'O' => Cell::RollingRock,
        '#' => Cell::StaticRock,
        '.' => Cell::Empty,
        _ => panic!("Invalid cell char: {}", c),
    }
}

fn parse_input() -> Grid2<Cell> {
    parse_grid2(INPUT)
}

fn print_grid(grid: &Grid2<Cell>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Pos2::new(x, y);
            let cell = grid.get(pos).unwrap();
            let c = match cell {
                Cell::RollingRock => 'O',
                Cell::StaticRock => '#',
                Cell::Empty => '.',
            };
            print!("{}", c);
        }
        println!();
    }
}

// Returns if anything changed
fn move_rolling_rocks_in_dir(grid: &mut Grid2<Cell>, dir: Vec2<i32>) -> bool {
    // Iterate over every cell, if it's a rolling rock then try to move it in the given direction
    // if it's empty there, otherwise do nothing

    let mut anything_changed = false;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Pos2::new(x as i32, y as i32);
            let cell = grid.get(pos).unwrap();
            if *cell != Cell::RollingRock {
                continue;
            }

            let new_pos = pos + dir;
            if !grid.is_in_bounds(new_pos) {
                continue;
            }

            let new_cell = grid.get(new_pos).unwrap();
            if *new_cell != Cell::Empty {
                continue;
            }

            // Move the rock
            grid[pos] = Cell::Empty;
            grid[new_pos] = Cell::RollingRock;
            anything_changed = true;
        }
    }

    anything_changed
}

fn move_rocks_until_settled(grid: &mut Grid2<Cell>, dir: Vec2<i32>) {
    while move_rolling_rocks_in_dir(grid, dir) {}
}

fn part1() {
    let mut input = parse_input();

    // Move rocks up
    move_rocks_until_settled(&mut input, Vec2::new(0, -1));

    let mut sum = 0;
    for y in 0..input.height() {
        for x in 0..input.width() {
            let pos = Pos2::new(x, y);
            let cell = input.get(pos).unwrap();
            if *cell != Cell::RollingRock {
                continue;
            }

            let load = input.height() - y;
            sum += load;
        }
    }

    print_grid(&input);

    println!("Part 1: {}", sum)
}

fn part2() {
    let mut input = parse_input();

    let mut past_inputs = vec![];

    let mut i = 0;

    // Move rocks up
    loop {
        move_rocks_until_settled(&mut input, Vec2::new(0, -1));
        move_rocks_until_settled(&mut input, Vec2::new(-1, 0));
        move_rocks_until_settled(&mut input, Vec2::new(0, 1));
        move_rocks_until_settled(&mut input, Vec2::new(1, 0));

        i += 1;
        dbg!(i);

        print_grid(&input);

        if past_inputs.contains(&input) {
            break;
        }

        past_inputs.push(input.clone());
    }

    let past_index = past_inputs.iter().position(|x| *x == input).unwrap();

    let cycle_len = past_inputs.len() - past_index;
    // Do 1000000000 cycles
    let offset = (1000000000 - past_index - 1) % cycle_len;

    input = past_inputs[past_index + offset].clone();

    let mut sum = 0;
    for y in 0..input.height() {
        for x in 0..input.width() {
            let pos = Pos2::new(x, y);
            let cell = input.get(pos).unwrap();
            if *cell != Cell::RollingRock {
                continue;
            }

            let load = input.height() - y;
            sum += load;
        }
    }

    print_grid(&input);

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
