use shared::*;

const INPUT: &str = day_input!();

// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Star,
    Space,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Space
    }
}

fn cell_from_char(c: char) -> Cell {
    match c {
        '#' => Cell::Star,
        '.' => Cell::Space,
        _ => panic!("Invalid cell"),
    }
}

fn parse_input_vecs() -> Vec<Vec<char>> {
    INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

fn parse_input() -> Grid2<Cell> {
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

struct StarGrid {
    stars: Vec<Pos2<i64>>,
}

impl StarGrid {
    pub fn from_grid(grid: &Grid2<Cell>) -> Self {
        let mut stars = Vec::new();

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid[Pos2::new(x, y)] == Cell::Star {
                    stars.push(Pos2::new(x as i64, y as i64));
                }
            }
        }

        Self { stars }
    }

    fn insert_row(&mut self, row: usize) {
        for star in &mut self.stars {
            if star.y >= row as i64 {
                star.y += 1000000 - 1;
            }
        }
    }

    fn insert_col(&mut self, col: usize) {
        for star in &mut self.stars {
            if star.x >= col as i64 {
                star.x += 1000000 - 1;
            }
        }
    }
}

// Same grid, but with a row inserted full of space
fn insert_row(grid: &Grid2<Cell>, row: usize) -> Grid2<Cell> {
    let mut new_row = Grid2::new_default(grid.width(), grid.height() + 1);

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos_old = Pos2::new(x, y);
            let pos_new = if y < row {
                Pos2::new(x, y)
            } else {
                Pos2::new(x, y + 1)
            };

            new_row[pos_new] = grid[pos_old];
        }
    }

    new_row
}

// Same as above but column
fn insert_col(grid: &Grid2<Cell>, col: usize) -> Grid2<Cell> {
    let mut new_col = Grid2::new_default(grid.width() + 1, grid.height());

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos_old = Pos2::new(x, y);
            let pos_new = if x < col {
                Pos2::new(x, y)
            } else {
                Pos2::new(x + 1, y)
            };

            new_col[pos_new] = grid[pos_old];
        }
    }

    new_col
}

fn is_row_empty(grid: &Grid2<Cell>, row: usize) -> bool {
    for x in 0..grid.width() {
        if grid[Pos2::new(x, row)] == Cell::Star {
            return false;
        }
    }

    true
}

fn is_column_empty(grid: &Grid2<Cell>, col: usize) -> bool {
    for y in 0..grid.height() {
        if grid[Pos2::new(col, y)] == Cell::Star {
            return false;
        }
    }

    true
}

fn get_galaxy_coordinates(grid: &Grid2<Cell>) -> Vec<Pos2<i32>> {
    let mut coords = Vec::new();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid[Pos2::new(x, y)] == Cell::Star {
                coords.push(Pos2::new(x as i32, y as i32));
            }
        }
    }

    coords
}

fn part1() {
    let input = parse_input();

    // For each row and column, iterate in reverse, and double any empty rows/columns
    let mut grid = input.clone();
    for row in (0..grid.height()).rev() {
        if is_row_empty(&grid, row) {
            grid = insert_row(&grid, row);
        }
    }

    for col in (0..grid.width()).rev() {
        if is_column_empty(&grid, col) {
            grid = insert_col(&grid, col);
        }
    }

    // Draw the grid
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            print!(
                "{}",
                match grid[Pos2::new(x, y)] {
                    Cell::Star => '#',
                    Cell::Space => '.',
                }
            );
        }
        println!();
    }

    let galaxies = get_galaxy_coordinates(&grid);
    let mut distance_sum = 0;
    // Iterate over all pairs
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (a, b) = (galaxies[i], galaxies[j]);

            let dx = (b.x - a.x).abs();
            let dy = (b.y - a.y).abs();

            distance_sum += dx + dy;
        }
    }

    println!("Part 1: {}", distance_sum)
}

fn part2() {
    let input = parse_input();

    // For each row and column, iterate in reverse, and double any empty rows/columns
    let mut star_grid = StarGrid::from_grid(&input);
    let grid = input.clone();
    for row in (0..grid.height()).rev() {
        if is_row_empty(&grid, row) {
            star_grid.insert_row(row);
        }
    }

    for col in (0..grid.width()).rev() {
        if is_column_empty(&grid, col) {
            star_grid.insert_col(col);
        }
    }

    let galaxies = star_grid.stars;
    let mut distance_sum = 0;
    // Iterate over all pairs
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (a, b) = (galaxies[i], galaxies[j]);

            let dx = (b.x - a.x).abs();
            let dy = (b.y - a.y).abs();

            distance_sum += dx + dy;
        }
    }

    println!("Part 2: {}", distance_sum)
}

fn main() {
    part1();
    part2();
}
