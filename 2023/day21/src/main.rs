use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use shared::*;

const INPUT: &str = day_input!();

// ...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty, // .
    Bush,  // #
    Start, // S
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "#" => Ok(Cell::Bush),
            "S" => Ok(Cell::Start),
            _ => Err(()),
        }
    }
}

fn parse_input() -> Grid2<Cell> {
    parse_grid2(INPUT)
}

fn part1() {
    let mut input = parse_input();
    // Find Start and replace it with Empty
    let mut start = Pos2::new(0, 0);
    for x in 0..input.width() {
        for y in 0..input.height() {
            if input[Pos2::new(x, y)] == Cell::Start {
                start = Pos2::new(x as i32, y as i32);
                input[Pos2::new(x, y)] = Cell::Empty;
            }
        }
    }

    let target_step_count = 64;
    let mut positions_at_step_count = HashSet::from([start]);

    for _ in 0..target_step_count {
        let mut new_positions = HashSet::new();

        for pos in &positions_at_step_count {
            for new_pos in pos.iter_four_directions() {
                if !input.is_in_bounds(new_pos) {
                    continue;
                }

                if input[new_pos] == Cell::Empty {
                    new_positions.insert(new_pos);
                }
            }
        }

        positions_at_step_count = new_positions;
    }

    let result = positions_at_step_count.len();

    println!("Part 1: {}", result)
}

fn build_astar_map(seeds: Vec<(Pos2<i32>, u32)>, grid: &Grid2<Cell>) -> Grid2<u32> {
    let mut result = Grid2::from_fn(grid.width(), grid.height(), |_| u32::MAX);

    let mut queue = VecDeque::new();
    for (seed, dist) in seeds {
        queue.push_back((seed, dist));
    }

    while let Some((pos, step_count)) = queue.pop_front() {
        if !grid.is_in_bounds(pos) {
            continue;
        }

        if grid[pos] != Cell::Empty {
            continue;
        }

        if result[pos] <= step_count {
            continue;
        }

        result[pos] = step_count;

        for new_pos in pos.iter_four_directions() {
            queue.push_back((new_pos, step_count + 1));
        }
    }

    result
}

fn print_astar_grid(astar_map: &Grid2<u32>) {
    for x in 0..astar_map.width() {
        for y in 0..astar_map.height() {
            let astar = astar_map[Pos2::new(x, y)];
            // Pad with spaces
            let astar = if astar == u32::MAX {
                print!("  ");
            } else {
                print!("{:0>2}", astar);
            };
            print!(" ");
        }
        println!();
    }
}

fn sort_seeds_by_to_normalize(seeds: &mut Vec<Pos2<i32>>) {
    seeds.sort_by_key(|pos| (pos.x, pos.y));
}

fn mirror_edge_coord(pos: Pos2<i32>, grid: &Grid2<Cell>) -> Vec<Pos2<i32>> {
    // Edges get mirrored to the other side. Corner cases get 2 mirrors.
    let mut result = Vec::new();

    if pos.x == 0 {
        result.push(Pos2::new(grid.width() as i32 - 1, pos.y));
    }

    if pos.x == grid.width() as i32 - 1 {
        result.push(Pos2::new(0, pos.y));
    }

    if pos.y == 0 {
        result.push(Pos2::new(pos.x, grid.height() as i32 - 1));
    }

    if pos.y == grid.height() as i32 - 1 {
        result.push(Pos2::new(pos.x, 0));
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

struct EdgeConnection {
    seeds: Vec<(Pos2<i32>, u32)>,
    offset: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResultGridAndDirections {
    astar_map: Grid2<u32>,
    edge_seeds: [Vec<(Pos2<i32>, u32)>; 4],
}

fn process_grid(astar_grid: Grid2<u32>) -> ResultGridAndDirections {
    let top_edge_coords = (0..astar_grid.width())
        .map(|x| Pos2::new(x as i32, 0))
        .collect::<Vec<_>>();
    let bottom_edge_coords = (0..astar_grid.width())
        .map(|x| Pos2::new(x as i32, astar_grid.height() as i32 - 1))
        .collect::<Vec<_>>();
    let left_edge_coords = (0..astar_grid.height())
        .map(|y| Pos2::new(0, y as i32))
        .collect::<Vec<_>>();
    let right_edge_coords = (0..astar_grid.height())
        .map(|y| Pos2::new(astar_grid.width() as i32 - 1, y as i32))
        .collect::<Vec<_>>();

    let mut edge_seeds = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    let get_min_for_coords = |coords: &[Pos2<i32>]| {
        coords
            .iter()
            .map(|pos| astar_grid[*pos])
            .min()
            .unwrap_or(u32::MAX)
    };

    let top_min = get_min_for_coords(&top_edge_coords);
    for (pos, pos_reflect) in top_edge_coords.iter().zip(bottom_edge_coords.iter()) {
        edge_seeds[Direction::Up as usize].push((*pos_reflect, astar_grid[*pos] - top_min));
    }

    let bottom_min = get_min_for_coords(&bottom_edge_coords);
    for (pos, pos_reflect) in bottom_edge_coords.iter().zip(top_edge_coords.iter()) {
        edge_seeds[Direction::Down as usize].push((*pos_reflect, astar_grid[*pos] - bottom_min));
    }

    let left_min = get_min_for_coords(&left_edge_coords);
    for (pos, pos_reflect) in left_edge_coords.iter().zip(right_edge_coords.iter()) {
        edge_seeds[Direction::Left as usize].push((*pos_reflect, astar_grid[*pos] - left_min));
    }

    let right_min = get_min_for_coords(&right_edge_coords);
    for (pos, pos_reflect) in right_edge_coords.iter().zip(left_edge_coords.iter()) {
        edge_seeds[Direction::Right as usize].push((*pos_reflect, astar_grid[*pos] - right_min));
    }

    ResultGridAndDirections {
        astar_map: astar_grid,
        edge_seeds,
    }
}

fn part2() {
    let input = parse_input();

    let astar_map = build_astar_map(vec![(Pos2::new(0, 0), 0), (Pos2::new(5, 0), 1)], &input);
    print_astar_grid(&astar_map);

    let result = process_grid(astar_map);
    dbg!(&result);

    dbg!(mirror_edge_coord(Pos2::new(0, 0), &input));

    // let mut astar_cache

    println!("Part 2: {}", -1)
}

fn main() {
    part1();
    part2();
}
