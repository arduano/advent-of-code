use std::collections::VecDeque;

use shared::*;

const INPUT: &str = day_input!();

fn parse_input_vecs() -> Vec<Vec<char>> {
    INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn parse_input() -> Grid2<Cells> {
    let vecs = parse_input_vecs();

    let mut grid = Grid2::new_default(vecs[0].len(), vecs.len());

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Pos2::new(x, y);
            grid[pos] = cell_from_char(vecs[y][x]);
        }
    }

    grid
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cells {
    Vertical = b'|',
    Horizontal = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthEast = b'7',
    SouthWest = b'F',
    Ground = b'.',
    Start = b'S',
}

impl Default for Cells {
    fn default() -> Self {
        Cells::Ground
    }
}

fn cell_from_char(c: char) -> Cells {
    match c {
        '|' => Cells::Vertical,
        '-' => Cells::Horizontal,
        'L' => Cells::NorthEast,
        'J' => Cells::NorthWest,
        '7' => Cells::SouthWest,
        'F' => Cells::SouthEast,
        '.' => Cells::Ground,
        'S' => Cells::Start,
        _ => panic!("Invalid cell"),
    }
}

fn cell_to_dirs(cell: Cells) -> Vec<Vec2<i32>> {
    match cell {
        Cells::Vertical => vec![Vec2::new(0, 1), Vec2::new(0, -1)],
        Cells::Horizontal => vec![Vec2::new(1, 0), Vec2::new(-1, 0)],
        Cells::NorthEast => vec![Vec2::new(0, -1), Vec2::new(1, 0)],
        Cells::NorthWest => vec![Vec2::new(0, -1), Vec2::new(-1, 0)],
        Cells::SouthEast => vec![Vec2::new(0, 1), Vec2::new(1, 0)],
        Cells::SouthWest => vec![Vec2::new(0, 1), Vec2::new(-1, 0)],
        Cells::Ground => vec![],
        Cells::Start => vec![],
    }
}

fn find_grid_start(grid: &Grid2<Cells>) -> Pos2<i32> {
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Pos2::new(x, y);
            if grid[pos] == Cells::Start {
                return pos.map(|a| a as i32);
            }
        }
    }

    panic!("No start found")
}

fn part1() {
    let grid = parse_input();

    let mut distances = Grid2::<u32>::new_default(grid.width(), grid.height());

    let start = find_grid_start(&grid);

    let mut bfs_queue = VecDeque::new();

    for adjacent in start.iter_four_directions() {
        if adjacent.x < 0 || adjacent.y < 0 {
            continue;
        }

        let dirs = cell_to_dirs(grid[adjacent])
            .into_iter()
            .map(|dir| adjacent + dir)
            .to_vec();

        if dirs.contains(&start) {
            bfs_queue.push_back((1, adjacent));
        }
    }

    while let Some((dist, pos)) = bfs_queue.pop_front() {
        if distances[pos] != 0 {
            continue;
        }

        distances[pos] = dist;

        for dir in cell_to_dirs(grid[pos]) {
            let new_pos = pos + dir;

            if !grid.is_in_bounds(new_pos) {
                continue;
            }

            if grid[new_pos] == Cells::Ground {
                continue;
            }

            bfs_queue.push_back((dist + 1, new_pos));
        }
    }

    // Find maximum distance
    let mut max = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Pos2::new(x, y);
            if grid[pos] != Cells::Ground {
                max = max.max(distances[pos]);
            }
        }
    }

    println!("Part 1: {}", max)
}

fn part2() {
    let grid = parse_input();

    let start = find_grid_start(&grid);

    let mut bfs_queue = VecDeque::new();

    for adjacent in start.iter_four_directions() {
        if adjacent.x < 0 || adjacent.y < 0 {
            continue;
        }

        let dirs = cell_to_dirs(grid[adjacent])
            .into_iter()
            .map(|dir| adjacent + dir)
            .to_vec();

        dbg!(adjacent, grid[adjacent], &dirs);

        if dirs.contains(&start) {
            bfs_queue.push_back((1, adjacent));
        }
    }

    let mut cycle_tiles = Grid2::<bool>::new_default(grid.width() * 2 + 1, grid.height() * 2 + 1);

    fn coord_to_double(coord: Pos2<i32>) -> Pos2<i32> {
        Pos2::new(coord.x * 2 + 1, coord.y * 2 + 1)
    }

    while let Some((dist, pos)) = bfs_queue.pop_front() {
        if cycle_tiles[coord_to_double(pos)] {
            continue;
        }

        cycle_tiles[coord_to_double(pos)] = true;

        for dir in cell_to_dirs(grid[pos]) {
            let new_pos = pos + dir;

            if !cycle_tiles.is_in_bounds(coord_to_double(new_pos)) {
                continue;
            }

            println!("from {} to {}", pos, new_pos);

            let middle = (coord_to_double(pos) + coord_to_double(new_pos).to_vec()) / 2;
            cycle_tiles[middle] = true;

            bfs_queue.push_back((dist + 1, new_pos));
        }
    }

    let mut floodfill_tiles = Grid2::<bool>::new_default(cycle_tiles.width(), cycle_tiles.height());
    // Fill any tiles starting from 0,0 that aren't a cycle tile
    let mut bfs_queue = VecDeque::new();
    bfs_queue.push_back(Pos2::new(0, 0));

    while let Some(pos) = bfs_queue.pop_front() {
        if floodfill_tiles[pos] {
            continue;
        }

        floodfill_tiles[pos] = true;

        for new_pos in pos.iter_four_directions() {
            if !cycle_tiles.is_in_bounds(new_pos) {
                continue;
            }

            if cycle_tiles[new_pos] {
                continue;
            }

            bfs_queue.push_back(new_pos);
        }
    }

    // Count all non cycle non fill tiles
    let mut inner_tiles = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Pos2::new(x as i32, y as i32);
            if !cycle_tiles[coord_to_double(pos)] && !floodfill_tiles[coord_to_double(pos)] {
                inner_tiles += 1;
            }
        }
    }

    // Print cycle tiles as X and O
    for y in 0..cycle_tiles.height() {
        for x in 0..cycle_tiles.width() {
            let pos = Pos2::new(x, y);
            if floodfill_tiles[pos] {
                print!(" ");
            } else if cycle_tiles[pos] {
                print!("X");
            } else {
                print!("O");
            }
        }
        println!();
    }

    println!("Part 2: {}", inner_tiles)
}

fn main() {
    part1();
    part2();
}
