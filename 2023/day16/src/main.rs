use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use shared::*;

const INPUT: &str = day_input!();

// .|...\....
// |.-.\.....
// .....|-...
// ........|.
// ..........
// .........\
// ..../.\\..
// .-.-/..|..
// .|....-|.\
// ..//.|....

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,      // .
    Vertical,   // |
    Horizontal, // -
    Left,       // /
    Right,      // \
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn direction_to_vector(dir: Direction) -> Vec2<i32> {
    match dir {
        Direction::Up => Vec2::new(0, -1),
        Direction::Down => Vec2::new(0, 1),
        Direction::Left => Vec2::new(-1, 0),
        Direction::Right => Vec2::new(1, 0),
    }
}

fn cell_to_directions(cell: Cell, dir: Direction) -> Vec<Direction> {
    // If the beam encounters empty space (.), it continues in the same direction.
    // If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
    // If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
    // If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.

    match (cell, dir) {
        (Cell::Empty, _) => vec![dir],

        (Cell::Vertical, Direction::Up) => vec![Direction::Up],
        (Cell::Vertical, Direction::Down) => vec![Direction::Down],
        (Cell::Vertical, Direction::Left) => vec![Direction::Up, Direction::Down],
        (Cell::Vertical, Direction::Right) => vec![Direction::Up, Direction::Down],

        (Cell::Horizontal, Direction::Left) => vec![Direction::Left],
        (Cell::Horizontal, Direction::Right) => vec![Direction::Right],
        (Cell::Horizontal, Direction::Up) => vec![Direction::Left, Direction::Right],
        (Cell::Horizontal, Direction::Down) => vec![Direction::Left, Direction::Right],

        (Cell::Left, Direction::Up) => vec![Direction::Right],
        (Cell::Left, Direction::Down) => vec![Direction::Left],
        (Cell::Left, Direction::Left) => vec![Direction::Down],
        (Cell::Left, Direction::Right) => vec![Direction::Up],

        (Cell::Right, Direction::Up) => vec![Direction::Left],
        (Cell::Right, Direction::Down) => vec![Direction::Right],
        (Cell::Right, Direction::Left) => vec![Direction::Up],
        (Cell::Right, Direction::Right) => vec![Direction::Down],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Beam {
    pos: Pos2<i32>,
    dir: Direction,
}

fn cell_from_char(c: char) -> Cell {
    match c {
        '.' => Cell::Empty,
        '|' => Cell::Vertical,
        '-' => Cell::Horizontal,
        '/' => Cell::Left,
        '\\' => Cell::Right,
        _ => panic!("Invalid cell char: {}", c),
    }
}

fn parse_grid(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| line.chars().map(cell_from_char).collect())
        .collect()
}

fn parse_input() -> Grid2<Cell> {
    let vecs = parse_grid(INPUT);
    let width = vecs[0].len();
    let height = vecs.len();

    let mut grid = Grid2::new_default(width, height);

    for (y, row) in vecs.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let pos = Pos2::new(x, y);
            *grid.get_mut(pos).unwrap() = *cell;
        }
    }

    grid
}

fn get_affected_grid(
    grid: &Grid2<Cell>,
    start: Pos2<i32>,
    dir: Direction,
) -> Grid2<HashSet<Direction>> {
    let mut beams: VecDeque<_> = [Beam { pos: start, dir }].into();

    let mut affected_grid = Grid2::<HashSet<Direction>>::new_default(grid.width(), grid.height());

    while let Some(beam) = beams.pop_front() {
        let affected = affected_grid.get_mut(beam.pos).unwrap();
        if affected.contains(&beam.dir) {
            continue;
        }
        affected.insert(beam.dir);

        let cell = grid.get(beam.pos).unwrap();
        let dirs = cell_to_directions(*cell, beam.dir);

        for dir in dirs {
            let pos = beam.pos + direction_to_vector(dir);
            if !grid.is_in_bounds(pos) {
                continue;
            }

            let new_beam = Beam { pos, dir };

            if !beams.contains(&new_beam) {
                beams.push_back(new_beam);
            }
        }
    }

    affected_grid
}

fn affected_grid_count(affected_grid: &Grid2<HashSet<Direction>>) -> usize {
    let mut sum = 0;
    for y in 0..affected_grid.height() {
        for x in 0..affected_grid.width() {
            let pos = Pos2::new(x, y);
            let cell = affected_grid.get(pos).unwrap();
            if cell.len() > 0 {
                sum += 1;
            }
        }
    }
    sum
}

fn part1() {
    let grid = parse_input();

    let affected_grid = get_affected_grid(&grid, Pos2::new(0, 0), Direction::Right);

    let sum = affected_grid_count(&affected_grid);

    println!("Part 1: {}", sum)
}

fn part2() {
    let input = parse_input();

    let mut max_affected = 0;

    // As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)
    // So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

    // Try all possible starting positions
    for y in 0..input.height() {
        let grid1 = get_affected_grid(&input, Pos2::new(0, y as i32), Direction::Right);
        let grid2 = get_affected_grid(
            &input,
            Pos2::new(input.width() as i32 - 1, y as i32),
            Direction::Left,
        );

        let sum1 = affected_grid_count(&grid1);
        let sum2 = affected_grid_count(&grid2);

        max_affected = max_affected.max(sum1).max(sum2);
    }

    for x in 0..input.width() {
        let grid1 = get_affected_grid(&input, Pos2::new(x as i32, 0), Direction::Down);
        let grid2 = get_affected_grid(
            &input,
            Pos2::new(x as i32, input.height() as i32 - 1),
            Direction::Up,
        );

        let sum1 = affected_grid_count(&grid1);
        let sum2 = affected_grid_count(&grid2);

        max_affected = max_affected.max(sum1).max(sum2);
    }

    println!("Part 2: {}", max_affected)
}

fn main() {
    part1();
    part2();
}
