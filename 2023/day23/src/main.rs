use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use shared::*;

const INPUT: &str = day_input!();

// #.#####################
// #.......#########...###
// #######.#########.#.###
// ###.....#.>.>.###.#.###
// ###v#####.#v#.###.#.###
// ###.>...#.#.#.....#...#
// ###v###.#.#.#########.#
// ###...#.#.#.......#...#
// #####.#.#.#######.#.###
// #.....#.#.#.......#...#
// #.#####.#.#.#########v#
// #.#...#...#...###...>.#
// #.#.#v#######v###.###v#
// #...#.>.#...>.>.#.###.#
// #####v#.#.###v#.#.###.#
// #.....#...#...#.#.#...#
// #.#########.###.#.#.###
// #...###...#...#...#.###
// ###.###.#.###v#####v###
// #...#...#.#.>.>.#.>.###
// #.###.###.#.###.#.#v###
// #.....###...###...#...#
// #####################.#

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Path,         // .
    Forest,       // #
    Slope(Slope), // v
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slope {
    Up,    // ^
    Down,  // v
    Left,  // <
    Right, // >
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Path),
            "#" => Ok(Cell::Forest),
            "v" => Ok(Cell::Slope(Slope::Down)),
            "^" => Ok(Cell::Slope(Slope::Up)),
            "<" => Ok(Cell::Slope(Slope::Left)),
            ">" => Ok(Cell::Slope(Slope::Right)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    cells: HashSet<Pos2<i32>>,
    current: Pos2<i32>,
    steps: i32,
}

fn parse_input() -> Grid2<Cell> {
    parse_grid2(INPUT)
}

fn process_cell(pos: Pos2<i32>, cell: Cell) -> Option<(Pos2<i32>, i32)> {
    match cell {
        Cell::Path => Some((pos, 1)),
        Cell::Forest => None,
        Cell::Slope(slope) => match slope {
            Slope::Up => Some((pos + Vec2::new(0, -1), 2)),
            Slope::Down => Some((pos + Vec2::new(0, 1), 2)),
            Slope::Left => Some((pos + Vec2::new(-1, 0), 2)),
            Slope::Right => Some((pos + Vec2::new(1, 0), 2)),
        },
    }
}

fn process_cell_2(pos: Pos2<i32>, cell: Cell) -> Option<(Pos2<i32>, i32)> {
    match cell {
        Cell::Path | Cell::Slope(_) => Some((pos, 1)),
        Cell::Forest => None,
    }
}

fn part1() {
    let grid = parse_input();

    let start_coord = Pos2::new(1, 0);
    let end_coord = Pos2::new(grid.width() as i32 - 2, grid.height() as i32 - 1);

    // Find the longest possible path without repeating cells

    let mut paths = VecDeque::from(vec![Path {
        cells: HashSet::new(),
        current: start_coord,
        steps: 0,
    }]);

    let mut longest = 0;

    while let Some(path) = paths.pop_back() {
        for pos2 in path.current.iter_four_directions() {
            if !grid.is_in_bounds(pos2) {
                continue;
            }

            let pos2_fixed = process_cell(pos2, grid[pos2]);
            let Some((pos2, steps)) = pos2_fixed else {
                continue;
            };

            if !grid.is_in_bounds(pos2) {
                continue;
            }

            if path.cells.contains(&pos2) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.cells.insert(pos2);
            new_path.current = pos2;
            new_path.steps += steps;

            if pos2 == end_coord {
                longest = longest.max(new_path.steps);
            } else {
                paths.push_back(new_path);
            }
        }
    }

    println!("Part 1: {}", longest)
}

fn part2() {
    let grid = parse_input();

    let start_coord = Pos2::new(1, 0);
    let end_coord = Pos2::new(grid.width() as i32 - 2, grid.height() as i32 - 1);

    // Find the longest possible path without repeating cells

    let mut paths = VecDeque::from(vec![Path {
        cells: HashSet::new(),
        current: start_coord,
        steps: 0,
    }]);

    let mut longest = 0;

    while let Some(path) = paths.pop_back() {
        for pos2 in path.current.iter_four_directions() {
            if !grid.is_in_bounds(pos2) {
                continue;
            }

            let pos2_fixed = process_cell_2(pos2, grid[pos2]);
            let Some((pos2, steps)) = pos2_fixed else {
                continue;
            };

            if !grid.is_in_bounds(pos2) {
                continue;
            }

            if path.cells.contains(&pos2) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.cells.insert(pos2);
            new_path.current = pos2;
            new_path.steps += steps;

            if pos2 == end_coord {
                longest = longest.max(new_path.steps);
            } else {
                paths.push_back(new_path);
            }
        }
    }

    println!("Part 2: {}", longest)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    pos: Pos2<i32>,
    dir_index: usize,
}

impl Step {
    fn new(pos: Pos2<i32>) -> Self {
        Self { pos, dir_index: 0 }
    }

    fn next_dir(&mut self) -> Option<Pos2<i32>> {
        let dirs = [
            Vec2::new(0, -1),
            Vec2::new(-1, 0),
            Vec2::new(0, 1),
            Vec2::new(1, 0),
        ];

        if self.dir_index >= dirs.len() {
            return None;
        }

        let dir = dirs[self.dir_index];
        self.dir_index += 1;
        Some(self.pos + dir)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path2 {
    cells: HashSet<Pos2<i32>>,
    steps: Vec<Step>,
    current: Pos2<i32>,
}

impl Path2 {
    fn new(start: Pos2<i32>) -> Self {
        Self {
            cells: HashSet::from([start]),
            steps: vec![Step::new(start)],
            current: start,
        }
    }

    fn push_next(&mut self, grid: &Grid2<Cell>) -> bool {
        let step = self.steps.last_mut().unwrap();
        while let Some(next) = step.next_dir() {
            if !grid.is_in_bounds(next) {
                continue;
            }

            if grid[next] == Cell::Forest {
                continue;
            }

            if self.cells.contains(&next) {
                continue;
            }

            self.cells.insert(next);
            self.steps.push(Step::new(next));
            self.current = next;
            return true;
        }

        false
    }

    fn pop(&mut self) -> bool {
        let step = self.steps.pop().unwrap();
        let Some(last) = self.steps.last() else {
            return false;
        };

        self.cells.remove(&step.pos);
        self.current = last.pos;

        true
    }

    fn is_at_pos(&self, pos: Pos2<i32>) -> bool {
        self.current == pos
    }
}

fn mkstep(pos: Pos2<i32>) -> Step {
    Step { pos, dir_index: 0 }
}

fn part2_2() {
    let grid = parse_input();

    let start_coord = Pos2::new(1, 0);
    let end_coord = Pos2::new(grid.width() as i32 - 2, grid.height() as i32 - 1);

    let mut path = Path2::new(start_coord);

    let mut longest = 0;

    'outer: loop {
        if path.is_at_pos(end_coord) {
            if longest < path.steps.len() {
                println!("New longest: {}", path.steps.len());
                longest = path.steps.len();
            }

            if !path.pop() {
                break 'outer;
            }

            while !path.push_next(&grid) {
                if !path.pop() {
                    break 'outer;
                }
            }
        }

        if !path.push_next(&grid) {
            if !path.pop() {
                break 'outer;
            }
        }
    }

    println!("Part 2: {}", longest - 1)
}

fn main() {
    part1();
    part2_2();
}
