use shared::*;

const INPUT: &str = day_input!();

// 1,0,1~1,2,1
// 0,0,2~2,0,2
// 0,2,3~2,2,3
// 0,0,4~0,2,4
// 2,0,5~2,2,5
// 0,1,6~2,1,6
// 1,1,8~1,1,9

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3d {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3d {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube3d {
    pos1: Vec3d,
    pos2: Vec3d,
}

impl Cube3d {
    pub fn new(pos1: Vec3d, pos2: Vec3d) -> Self {
        Self {
            pos1: pos1.min(&pos2),
            pos2: pos1.max(&pos2),
        }
    }
}

fn parse_brick_line(line: &str) -> Cube3d {
    let mut parts = line.split('~');
    let pos1 = parts.next().unwrap();
    let pos2 = parts.next().unwrap();
    let mut pos1 = pos1.split(',');
    let mut pos2 = pos2.split(',');
    let x1 = pos1.next().unwrap().parse::<i32>().unwrap();
    let y1 = pos1.next().unwrap().parse::<i32>().unwrap();
    let z1 = pos1.next().unwrap().parse::<i32>().unwrap();
    let x2 = pos2.next().unwrap().parse::<i32>().unwrap();
    let y2 = pos2.next().unwrap().parse::<i32>().unwrap();
    let z2 = pos2.next().unwrap().parse::<i32>().unwrap();
    Cube3d::new(Vec3d::new(x1, y1, z1), Vec3d::new(x2, y2, z2))
}

fn parse_input() -> Vec<Cube3d> {
    INPUT
        .lines()
        .map(|line| parse_brick_line(line))
        .collect::<Vec<_>>()
}

fn offset_brick_by(brick: &Cube3d, offset: &Vec3d) -> Cube3d {
    Cube3d {
        pos1: Vec3d {
            x: brick.pos1.x + offset.x,
            y: brick.pos1.y + offset.y,
            z: brick.pos1.z + offset.z,
        },
        pos2: Vec3d {
            x: brick.pos2.x + offset.x,
            y: brick.pos2.y + offset.y,
            z: brick.pos2.z + offset.z,
        },
    }
}

fn do_bricks_overlap(brick1: &Cube3d, brick2: &Cube3d) -> bool {
    let x_overlap = brick1.pos1.x <= brick2.pos2.x && brick1.pos2.x >= brick2.pos1.x;
    let y_overlap = brick1.pos1.y <= brick2.pos2.y && brick1.pos2.y >= brick2.pos1.y;
    let z_overlap = brick1.pos1.z <= brick2.pos2.z && brick1.pos2.z >= brick2.pos1.z;
    x_overlap && y_overlap && z_overlap
}

fn is_brick_negative(brick: &Cube3d) -> bool {
    brick.pos1.z < 0
}

fn settle_bricks(bricks: &mut Vec<Cube3d>) {
    let mut settled_bricks = 0;
    while settled_bricks < bricks.len() {
        let mut curr_settled = 0;
        let mut new_bricks = Vec::new();
        for (i, b) in bricks.iter().enumerate() {
            let b_shifted = offset_brick_by(b, &Vec3d::new(0, 0, -1));

            if is_brick_negative(&b_shifted) {
                curr_settled += 1;
                new_bricks.push(*b);
                continue;
            }

            // Check for any overlaps
            let mut overlaps = false;
            for (j, b2) in bricks.iter().enumerate() {
                if i == j {
                    continue;
                }
                if do_bricks_overlap(&b_shifted, b2) {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                new_bricks.push(b_shifted);
            } else {
                new_bricks.push(*b);
                curr_settled += 1;
            }
        }

        *bricks = new_bricks;
        settled_bricks = curr_settled;
    }
}

fn are_bricks_unsettled(bricks: &Vec<Cube3d>) -> bool {
    for (i, b) in bricks.iter().enumerate() {
        let b_shifted = offset_brick_by(b, &Vec3d::new(0, 0, -1));

        if is_brick_negative(&b_shifted) {
            continue;
        }

        // Check for any overlaps
        let mut overlaps = false;
        for (j, b2) in bricks.iter().enumerate() {
            if i == j {
                continue;
            }
            if do_bricks_overlap(&b_shifted, b2) {
                overlaps = true;
                break;
            } else {
            }
        }

        if !overlaps {
            return true;
        }
    }

    false
}

fn number_neq<T: Eq>(vec1: &Vec<T>, vec2: &Vec<T>) -> usize {
    let mut neq = 0;
    for (i, v) in vec1.iter().enumerate() {
        if v != &vec2[i] {
            neq += 1;
        }
    }
    neq
}

fn part1() {
    let mut bricks = parse_input();

    settle_bricks(&mut bricks);

    // dbg!(bricks[0]);
    // dbg!(bricks[1]);

    // let mut bricks_removed = 0;
    // loop {
    //     let mut brick_removed = false;
    //     for i in 0..bricks.len() {
    //         let mut new_bricks = bricks.clone();
    //         new_bricks.remove(i);
    //         println!("Trying to remove brick {}", i);
    //         if !are_bricks_unsettled(&mut new_bricks) {
    //             println!("Removed brick {}", i);
    //             bricks = new_bricks;
    //             bricks_removed += 1;
    //             brick_removed = true;
    //             break;
    //         }
    //     }

    //     if !brick_removed {
    //         break;
    //     }
    // }

    let mut bricks_removed = 0;
    for i in 0..bricks.len() {
        let mut new_bricks = bricks.clone();
        new_bricks.remove(i);
        if !are_bricks_unsettled(&mut new_bricks) {
            bricks_removed += 1;
        }
    }

    println!("Part 1: {}", bricks_removed)
}

fn part2() {
    let mut bricks = parse_input();

    settle_bricks(&mut bricks);

    let mut sum_fallen = 0;
    for i in 0..bricks.len() {
        let mut new_bricks = bricks.clone();
        new_bricks.remove(i);
        let mut bricks_settled = new_bricks.clone();
        settle_bricks(&mut bricks_settled);

        sum_fallen += number_neq(&new_bricks, &bricks_settled);
    }

    println!("Part 2: {}", sum_fallen)
}

fn main() {
    // part1();
    part2();
}
