use shared::*;

const INPUT: &str = day_input!();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,  // .
    Rock, // #
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Ash
    }
}

fn parse_block(block: &str) -> Grid2<Tile> {
    let width = block.lines().next().unwrap().len();
    let height = block.lines().count();
    let mut grid = Grid2::new_default(width, height);

    for (y, line) in block.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Pos2::new(x, y);
            *grid.get_mut(pos).unwrap() = match c {
                '.' => Tile::Ash,
                '#' => Tile::Rock,
                _ => panic!("Invalid tile char: {}", c),
            };
        }
    }

    grid
}

fn rows_differences(grid: &Grid2<Tile>, row1: usize, row2: usize) -> u32 {
    let mut sum = 0;
    for x in 0..grid.width() {
        if grid.get(Pos2::new(x, row1)).unwrap() != grid.get(Pos2::new(x, row2)).unwrap() {
            sum += 1;
        }
    }

    sum
}

fn do_differences(grid: &Grid2<Tile>, col1: usize, col2: usize) -> u32 {
    let mut sum = 0;
    for y in 0..grid.height() {
        if grid.get(Pos2::new(col1, y)).unwrap() != grid.get(Pos2::new(col2, y)).unwrap() {
            sum += 1;
        }
    }

    sum
}

fn rows_diff_count_after(grid: &Grid2<Tile>, row: usize) -> u32 {
    let remaining_up = grid.height() - row - 2;
    let remianing_down = row;
    let remaining = remaining_up.min(remianing_down);

    let mut sum = 0;
    for y in 0..=remaining {
        sum += rows_differences(grid, row + y + 1, row - y);
    }

    sum
}

fn cols_diff_count_after(grid: &Grid2<Tile>, col: usize) -> u32 {
    let remaining_left = grid.width() - col - 2;
    let remaining_right = col;
    let remaining = remaining_left.min(remaining_right);

    let mut sum = 0;
    for x in 0..=remaining {
        sum += do_differences(grid, col + x + 1, col - x);
    }

    sum
}

fn find_row_reflection(grid: &Grid2<Tile>) -> Option<usize> {
    for y in 0..grid.height() - 1 {
        if rows_diff_count_after(grid, y) == 0 {
            return Some(y);
        }
    }

    None
}

fn find_col_reflection(grid: &Grid2<Tile>) -> Option<usize> {
    for x in 0..grid.width() - 1 {
        if cols_diff_count_after(grid, x) == 0 {
            return Some(x);
        }
    }

    None
}

fn find_row_partial_reflection(grid: &Grid2<Tile>) -> Option<usize> {
    for y in 0..grid.height() - 1 {
        if rows_diff_count_after(grid, y) == 1 {
            return Some(y);
        }
    }

    None
}

fn find_col_partial_reflection(grid: &Grid2<Tile>) -> Option<usize> {
    for x in 0..grid.width() - 1 {
        if cols_diff_count_after(grid, x) == 1 {
            return Some(x);
        }
    }

    None
}

fn parse_input() -> Vec<Grid2<Tile>> {
    INPUT.split("\n\n").map(parse_block).collect()
}

fn part1() {
    let input = parse_input();

    let mut sum = 0;

    for grid in input {
        let row_reflection = find_row_reflection(&grid);
        let col_reflection = find_col_reflection(&grid);

        sum += row_reflection.map(|i| i + 1).unwrap_or_default() * 100;
        sum += col_reflection.map(|i| i + 1).unwrap_or_default();
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let input = parse_input();

    let mut sum = 0;

    for grid in input {
        let row_reflection = find_row_partial_reflection(&grid);
        let col_reflection = find_col_partial_reflection(&grid);

        sum += row_reflection.map(|i| i + 1).unwrap_or_default() * 100;
        sum += col_reflection.map(|i| i + 1).unwrap_or_default();
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
