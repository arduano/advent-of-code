use std::collections::{BinaryHeap, VecDeque};

use shared::*;

const INPUT: &str = day_input!();

fn parse_input() -> Grid2<u32> {
    parse_grid2(INPUT)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    pos: Pos2<i32>,
    dir: Vec2<i32>,
    moved_consecutive: u32,
    heat_lost: u32,
    prev_pos: Vec<Pos2<i32>>,
    total_steps: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PositionContainer(Position);

impl Ord for PositionContainer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_dist = self.0.pos.x + self.0.pos.y;
        let other_dist = other.0.pos.x + other.0.pos.y;

        other_dist.cmp(&self_dist)
    }
}

impl PartialOrd for PositionContainer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Position {
    fn new(pos: Pos2<i32>, dir: Vec2<i32>) -> Self {
        Self {
            pos,
            dir,
            moved_consecutive: 0,
            heat_lost: 0,
            prev_pos: Vec::new(),
            total_steps: 0,
        }
    }

    fn add_heat(&mut self, input: &Grid2<u32>) {
        let heat = input.get(self.pos).unwrap();
        self.heat_lost += heat;
    }

    fn walk_forward(&self, input: &Grid2<u32>) -> Option<Self> {
        let mut new = self.clone();

        new.prev_pos.push(new.pos);
        new.pos += new.dir;

        if !input.is_in_bounds(new.pos) {
            return None;
        }

        new.add_heat(input);
        new.moved_consecutive += 1;
        new.total_steps += 1;

        if new.can_go_forward() {
            Some(new)
        } else {
            None
        }
    }

    fn walk_left(&self, input: &Grid2<u32>) -> Option<Self> {
        let mut new = self.clone();

        new.prev_pos.push(new.pos);
        new.dir = Vec2::new(-new.dir.y, new.dir.x);
        new.pos += new.dir;

        if !input.is_in_bounds(new.pos) {
            return None;
        }

        new.add_heat(input);
        new.moved_consecutive = 1;
        new.total_steps += 1;

        Some(new)
    }

    fn walk_right(&self, input: &Grid2<u32>) -> Option<Self> {
        let mut new = self.clone();

        new.prev_pos.push(new.pos);
        new.dir = Vec2::new(new.dir.y, -new.dir.x);
        new.pos += new.dir;

        if !input.is_in_bounds(new.pos) {
            return None;
        }

        new.add_heat(input);
        new.moved_consecutive = 1;
        new.total_steps += 1;

        Some(new)
    }

    fn can_go_forward(&self) -> bool {
        self.moved_consecutive < 4
    }
}

fn print_grid_with_path(grid: &Grid2<u32>, min_path: &[Pos2<i32>]) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Pos2::new(x as i32, y as i32);
            if min_path.contains(&pos) {
                print!("X");
            } else {
                print!("{}", grid.get(pos).unwrap());
            }
        }
        println!();
    }
}

fn dir_to_cell(dir: Vec2<i32>) -> usize {
    // Each of the 4 directions gets assigned an int
    if dir == Vec2::new(0, -1) {
        0
    } else if dir == Vec2::new(0, 1) {
        1
    } else if dir == Vec2::new(-1, 0) {
        2
    } else if dir == Vec2::new(1, 0) {
        3
    } else {
        panic!("Invalid direction: {:?}", dir);
    }
}

fn part1() {
    let grid = parse_input();

    let mut min_grid_reached = Grid2::new_with(grid.width(), grid.height(), [[u32::MAX; 4]; 4]);

    // BFS
    let mut queue = BinaryHeap::new();
    queue.push(PositionContainer(Position::new(
        Pos2::new(0, 0),
        Vec2::new(1, 0),
    )));

    let winning_pos = Pos2::new(grid.width() as i32 - 1, grid.height() as i32 - 1);

    let mut max_heat = u32::MAX;
    let mut min_path = vec![];

    while let Some(PositionContainer(position)) = queue.pop() {
        // println!("Max: {:?}", queue.iter().map(|p| p.0.total_steps).max());
        if winning_pos == position.pos {
            if position.heat_lost < max_heat {
                println!("Found winning position: {:?}", position.heat_lost);
                max_heat = position.heat_lost;
                min_path = position.prev_pos;
            }
            continue;
        }

        if !grid.is_in_bounds(position.pos) {
            continue;
        }

        if position.heat_lost > max_heat {
            continue;
        }

        let min_cell_row = &mut min_grid_reached[position.pos];
        let min_cell =
            &mut min_cell_row[dir_to_cell(position.dir)][position.moved_consecutive as usize];
        if position.heat_lost >= *min_cell {
            continue;
        }
        *min_cell = position.heat_lost;

        let new_positions = [
            position.walk_forward(&grid),
            position.walk_left(&grid),
            position.walk_right(&grid),
        ];

        for new_position in new_positions.into_iter().flatten() {
            queue.push(PositionContainer(new_position));
        }
    }

    // Debug min path
    print_grid_with_path(&grid, &min_path);

    println!("Part 1: {}", max_heat)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position2 {
    pos: Pos2<i32>,
    dir: Vec2<i32>,
    moved_consecutive: u32,
    heat_lost: u32,
    prev_pos: Vec<Pos2<i32>>,
    total_steps: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position2Container(Position2);

impl Ord for Position2Container {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_dist = self.0.pos.x + self.0.pos.y;
        let other_dist = other.0.pos.x + other.0.pos.y;

        other_dist.cmp(&self_dist)
    }
}

impl PartialOrd for Position2Container {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Position2 {
    fn new(pos: Pos2<i32>, dir: Vec2<i32>) -> Self {
        Self {
            pos,
            dir,
            moved_consecutive: 0,
            heat_lost: 0,
            prev_pos: Vec::new(),
            total_steps: 0,
        }
    }

    fn add_heat(&mut self, input: &Grid2<u32>) {
        let heat = input.get(self.pos).unwrap();
        self.heat_lost += heat;
    }

    fn walk_forward(&self, input: &Grid2<u32>) -> Option<Self> {
        let mut new = self.clone();

        if !self.can_go_forward() {
            return None;
        }

        new.prev_pos.push(new.pos);
        new.pos += new.dir;

        if !input.is_in_bounds(new.pos) {
            return None;
        }

        new.add_heat(input);
        new.moved_consecutive += 1;
        new.total_steps += 1;

        Some(new)
    }

    fn walk_left(&self, input: &Grid2<u32>) -> Option<Self> {
        let mut new = self.clone();

        if !self.can_turn() {
            return None;
        }

        new.prev_pos.push(new.pos);
        new.dir = Vec2::new(-new.dir.y, new.dir.x);
        new.pos += new.dir;

        if !input.is_in_bounds(new.pos) {
            return None;
        }

        new.add_heat(input);
        new.moved_consecutive = 1;
        new.total_steps += 1;

        Some(new)
    }

    fn walk_right(&self, input: &Grid2<u32>) -> Option<Self> {
        let mut new = self.clone();

        if !self.can_turn() {
            return None;
        }

        new.prev_pos.push(new.pos);
        new.dir = Vec2::new(new.dir.y, -new.dir.x);
        new.pos += new.dir;

        if !input.is_in_bounds(new.pos) {
            return None;
        }

        new.add_heat(input);
        new.moved_consecutive = 1;
        new.total_steps += 1;

        Some(new)
    }

    fn can_go_forward(&self) -> bool {
        self.moved_consecutive < 10
    }

    fn can_turn(&self) -> bool {
        self.moved_consecutive > 3
    }

    fn can_stop(&self) -> bool {
        self.moved_consecutive > 3
    }
}

fn part2() {
    let grid = parse_input();

    let mut min_grid_reached = Grid2::new_with(grid.width(), grid.height(), [[u32::MAX; 11]; 4]);

    // BFS
    let mut queue = BinaryHeap::new();
    queue.push(Position2Container(Position2::new(
        Pos2::new(0, 0),
        Vec2::new(1, 0),
    )));
    queue.push(Position2Container(Position2::new(
        Pos2::new(0, 0),
        Vec2::new(0, 1),
    )));

    let winning_pos = Pos2::new(grid.width() as i32 - 1, grid.height() as i32 - 1);

    let mut max_heat = u32::MAX;
    let mut min_path = vec![];

    while let Some(Position2Container(position)) = queue.pop() {
        // println!("Max: {:?}", queue.iter().map(|p| p.0.total_steps).max());
        if winning_pos == position.pos {
            if !position.can_stop() {
                continue;
            }
            if position.heat_lost < max_heat {
                println!("Found winning position: {:?}", position.heat_lost);
                max_heat = position.heat_lost;
                min_path = position.prev_pos;
            }
            continue;
        }

        if !grid.is_in_bounds(position.pos) {
            continue;
        }

        if position.heat_lost > max_heat {
            continue;
        }

        let min_cell_row = &mut min_grid_reached[position.pos];
        let min_cell =
            &mut min_cell_row[dir_to_cell(position.dir)][position.moved_consecutive as usize];
        if position.heat_lost >= *min_cell {
            continue;
        }
        *min_cell = position.heat_lost;

        let new_positions = [
            position.walk_forward(&grid),
            position.walk_left(&grid),
            position.walk_right(&grid),
        ];

        for new_position in new_positions.into_iter().flatten() {
            queue.push(Position2Container(new_position));
        }
    }

    // Debug min path
    print_grid_with_path(&grid, &min_path);

    println!("Part 2: {}", max_heat)
}

fn main() {
    part1();
    part2();
}
