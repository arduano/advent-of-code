use shared::*;

const INPUT: &str = day_input!();

fn is_valid_symbol(c: char) -> bool {
    // c is not:
    // - a digit
    // - a period
    // - null

    !c.is_ascii_digit() && c != '.' && c != '\0'
}

fn part1() {
    let lines = parse_lines::<String>(INPUT);

    let width = lines[0].len();

    let mut grid = IGrid2::<char>::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[Pos2::new(x, y)] = c;
        }
    }

    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let y = y as isize;
        let mut x = 0isize;
        while x < width as isize {
            let coord = Pos2::new(x, y);

            let remaining = &line[(x as usize)..];
            // Check if a number starts at this char
            let length = remaining.chars().take_while(|c| c.is_ascii_digit()).count() as isize;

            if length == 0 {
                x += 1;
                continue;
            }

            let number = remaining[..(length as usize)].parse::<usize>().unwrap();

            // Iterate over every single adjacent cell
            let mut has_adjacent = false;
            'outer: for x2 in -1..=length {
                for y2 in -1..=1 {
                    if y2 == 0 && (x2 >= 0 && x2 < length) {
                        continue;
                    }

                    let coord2 = coord + Vec2::new(x2, y2);

                    let char = grid.get(coord2);

                    if is_valid_symbol(*char) {
                        has_adjacent = true;
                        break 'outer;
                    }
                }
            }

            if has_adjacent {
                sum += number;
            }

            x += length;
        }
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let lines = parse_lines::<String>(INPUT);

    let width = lines[0].len();
    let height = lines.len();

    let mut grid = IGrid2::<char>::new();
    let mut gear_ratiod = IGrid2::<Vec<u32>>::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[Pos2::new(x, y)] = c;
        }
    }

    for (y, line) in lines.iter().enumerate() {
        let y = y as isize;
        let mut x = 0isize;
        while x < width as isize {
            let coord = Pos2::new(x, y);

            let remaining = &line[(x as usize)..];
            // Check if a number starts at this char
            let length = remaining.chars().take_while(|c| c.is_ascii_digit()).count() as isize;

            if length == 0 {
                x += 1;
                continue;
            }

            let number = remaining[..(length as usize)].parse::<usize>().unwrap();

            // Iterate over every single adjacent cell
            for x2 in -1..=length {
                for y2 in -1..=1 {
                    if y2 == 0 && (x2 >= 0 && x2 < length) {
                        continue;
                    }

                    let coord2 = coord + Vec2::new(x2, y2);

                    let char = grid.get(coord2);

                    if *char == '*' {
                        gear_ratiod[coord2].push(number as u32);
                    }
                }
            }

            x += length;
        }
    }

    let mut sum = 0;
    for x in 0..width {
        for y in 0..height {
            let coord = Pos2::new(x, y);

            let ratios = &gear_ratiod[coord];
            if ratios.len() == 2 {
                sum += ratios[0] * ratios[1];
            }
        }
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
