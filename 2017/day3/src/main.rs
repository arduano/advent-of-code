use shared::*;

const INPUT: &str = day_input!();

fn iter_side(from: Pos2<i32>, add: Vec2<i32>, length: i32) -> impl Iterator<Item = Pos2<i32>> {
    (0..length).map(move |i| from + add * i)
}

fn iter_shell(n: i32) -> impl Iterator<Item = Pos2<i32>> {
    let start = Pos2::new(n, -n + 1);

    let length = n * 2;

    let side1 = iter_side(start, Vec2::up(), length);
    let side2 = iter_side(start.rot_left(), Vec2::left(), length);
    let side3 = iter_side(start.flip(), Vec2::down(), length);
    let side4 = iter_side(start.rot_right(), Vec2::right(), length);

    side1.chain(side2).chain(side3).chain(side4)
}

fn iter_coords() -> impl Iterator<Item = Pos2<i32>> {
    let first = std::iter::once(Pos2::zero());
    let rest = (1..).flat_map(|n| iter_shell(n));

    first.chain(rest)
}

fn part1() {
    let index = INPUT.parse::<usize>().unwrap();
    let coord = iter_coords().nth(index - 1).unwrap();
    let dist = coord.x.abs() + coord.y.abs();

    println!("Part 1: {}", dist)
}

fn part2() {
    let input = INPUT.parse::<u32>().unwrap();
    // let input = 747;

    let mut grid = IGrid2::<u32>::new();

    let mut val = None;

    for c in iter_coords() {
        let mut sum = 0;
        for n in c.iter_eight_directions() {
            sum += grid[n];
        }

        if sum == 0 {
            sum = 1;
        }

        grid[c] = sum;

        if sum > input {
            val = Some(sum);
            break;
        }
    }

    println!("Part 2: {}", val.unwrap())
}

fn main() {
    part1();
    part2();
}
