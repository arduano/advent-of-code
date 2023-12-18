use std::collections::VecDeque;

use shared::*;

const INPUT: &str = day_input!();

// R 6 (#70c710)
// D 5 (#0dc571)
// L 2 (#5713f0)
// D 2 (#d2c081)
// R 2 (#59c680)
// D 2 (#411b91)
// L 5 (#8ceee2)
// U 2 (#caa173)
// L 1 (#1b58a2)
// U 2 (#caa171)
// R 2 (#7807d2)
// U 3 (#a77fa3)
// L 2 (#015232)
// U 2 (#7a21e3)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn clockwise(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

fn dir_to_vec(dir: Dir) -> Vec2<i32> {
    match dir {
        Dir::Up => Vec2::new(0, -1),
        Dir::Down => Vec2::new(0, 1),
        Dir::Left => Vec2::new(-1, 0),
        Dir::Right => Vec2::new(1, 0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    dir: Dir,
    steps: usize,

    dir2: Dir,
    steps2: u32,
}

fn parse_lines(line: &str) -> Vec<Move> {
    line.lines()
        .map(|s| {
            let mut parts = s.split_whitespace();
            let dir = match parts.next().unwrap() {
                "R" => Dir::Right,
                "L" => Dir::Left,
                "U" => Dir::Up,
                "D" => Dir::Down,
                _ => unreachable!(),
            };

            let steps = parts.next().unwrap().parse().unwrap();

            let color_str = parts.next().unwrap();

            let dir2 = match color_str.chars().nth(7).unwrap() {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                _ => unreachable!(),
            };

            Move {
                dir,
                steps,
                steps2: u32::from_str_radix(&color_str[2..7], 16).unwrap(),
                dir2,
            }
        })
        .collect()
}

fn part1() {
    let input = parse_lines(INPUT);

    let mut affected = IGrid2::<bool>::new_with_default();

    let mut pos = Pos2::new(0, 0);

    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    // Follow and trace paths
    for m in input {
        let dir = dir_to_vec(m.dir);
        for _ in 0..m.steps {
            pos += dir;
            affected[pos] = true;
        }

        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
    }

    let start = Pos2::new(1, 1);
    let mut queue = VecDeque::new();
    queue.push_back(start);

    // Floodfill
    while let Some(pos) = queue.pop_front() {
        for dir in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let new_pos = pos + dir_to_vec(*dir);
            if !affected[new_pos] {
                affected[new_pos] = true;
                queue.push_back(new_pos);
            }
        }
    }

    // Sum of all affected tiles
    let mut sum = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if affected[Pos2::new(x, y)] {
                sum += 1;
            }
        }
    }

    // Print the grid
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if affected[Pos2::new(x, y)] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("Part 1: {}", sum)
}

fn part2() {
    let input = parse_lines(INPUT);

    // let mut pos = Pos2::new(0, 0);

    // let mut row_dirs: BTreeMap<i64, BTreeMap<i64, Dir>> = BTreeMap::new();
    // let mut add_pos_dir = |pos: Pos2<i32>, dir: Dir| {
    //     let row = row_dirs.entry(pos.y as i64).or_default();
    //     row.insert(pos.x as i64, dir);
    // };

    // // Follow and trace paths
    // for m in input {
    //     let dir = dir_to_vec(m.dir);

    //     let add_dirs = m.dir == Dir::Up || m.dir == Dir::Down;
    //     if add_dirs {
    //         add_pos_dir(pos, m.dir);
    //     }
    //     for _ in 0..m.steps {
    //         pos += dir;

    //         if add_dirs {
    //             add_pos_dir(pos, m.dir);
    //         }
    //     }
    // }

    // let mut sum = 0;
    // // Count distances between up and down lines on each row
    // for (y, row) in row_dirs.iter() {
    //     let mut groups = Vec::new();
    //     // Group by directions
    //     let mut x_group = Vec::new();
    //     let mut group_dir = None;
    //     for (x, dir) in row.iter() {
    //         if group_dir.is_none() {
    //             group_dir = Some(*dir);
    //         }

    //         if *dir != group_dir.unwrap() {
    //             groups.push((group_dir.unwrap(), x_group));
    //             x_group = Vec::new();
    //             group_dir = Some(*dir);
    //         }

    //         x_group.push(*x);
    //     }
    //     groups.push((group_dir.unwrap(), x_group));

    //     let mut inside = false;
    //     let mut prev_x = 0;
    //     let mut sum2 = 0;
    //     for (_, xs) in groups.iter() {
    //         if !inside {
    //             inside = true;
    //             prev_x = *xs.first().unwrap();
    //         } else {
    //             inside = false;
    //             sum2 += (xs.last().unwrap() - prev_x) + 1;
    //         }
    //     }

    //     sum += sum2;
    // }

    // Do gauss area formula
    let mut area_sum = 0i64;
    let mut prev_pos = Pos2::new(0, 0);

    for i in 0..input.len() {
        let m = &input[i];
        let m_next = &input[(i + 1) % input.len()];
        let m_prev = if i == 0 {
            &input[input.len() - 1]
        } else {
            &input[i - 1]
        };

        let dir = dir_to_vec(m.dir2);

        let mut pos = prev_pos;

        if m_prev.dir2.clockwise() == m.dir2 {
            // println!("Adding first");
            if m.dir2 == Dir::Up {
                area_sum -= pos.x as i64;
            } else if m.dir2 == Dir::Down {
                area_sum += pos.x as i64 + 1;
            }
        }

        for _ in 0..m.steps2 - 1 {
            pos += dir;

            if m.dir2 == Dir::Up {
                area_sum -= pos.x as i64;
            } else if m.dir2 == Dir::Down {
                area_sum += pos.x as i64 + 1;
            }
        }
        pos += dir;

        if m.dir2.clockwise() == m_next.dir2 {
            // println!("Adding last");
            if m.dir2 == Dir::Up {
                area_sum -= pos.x as i64;
            } else if m.dir2 == Dir::Down {
                area_sum += pos.x as i64 + 1;
            }
        }

        // dbg!(pos.x);
        // dbg!(&m);
        // dbg!(area_sum);

        prev_pos = pos;
    }

    println!("Part 2: {}", area_sum)
}

fn main() {
    part1();
    part2();
}
