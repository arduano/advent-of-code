use std::ops::Range;

use shared::*;

const INPUT: &str = day_input!();

#[derive(Debug)]
struct MapRange {
    start_range: Range<u64>,
    dest_range: Range<u64>,
}

#[derive(Debug, Clone)]
struct ValueRanges {
    ranges: Vec<Range<u64>>,
}

fn range_len(range: &Range<u64>) -> u64 {
    range.end - range.start
}

/// Returns: old, new
fn split_range_at(
    range: Range<u64>,
    at: Range<u64>,
    new_start: u64,
) -> (Vec<Range<u64>>, Option<Range<u64>>) {
    // 5 conditions:
    // Range no overlap: return original range
    // Full overlap: return new mapped range
    // Left overlap: split out the left half and map it, 2 ranges
    // Right overlap: split out the right half and map it, 2 ranges
    // Center overlap: split out the center and map it, 3 ranges

    if range.start >= at.end || range.end <= at.start {
        // No overlap
        (vec![range], None)
    } else if range.start >= at.start && range.end <= at.end {
        // Full overlap
        let offset = range.start - at.start;
        (
            vec![],
            Some((new_start + offset)..(new_start + offset + range_len(&range))),
        )
    } else if range.start < at.start && range.end <= at.end {
        // Right overlap
        let overlap_len = range.end - at.start;
        let left = range.start..at.start;
        let right = new_start..(new_start + overlap_len);

        (vec![left], Some(right))
    } else if range.start >= at.start && range.end > at.end {
        // Left overlap
        let overlap_len = at.end - range.start;
        let offset = range.start - at.start;
        let left = (new_start + offset)..(new_start + offset + overlap_len);
        let right = at.end..range.end;

        (vec![right], Some(left))
    } else {
        // Center overlap
        let left = range.start..at.start;
        let center = new_start..(new_start + range_len(&at));
        let right = at.end..range.end;

        (vec![left, right], Some(center))
    }
}

impl ValueRanges {
    fn new(mut ranges: Vec<Range<u64>>) -> Self {
        ranges.sort_by_key(|range| range.start);
        // Map, joining overlapping ranges

        let mut new_ranges = Vec::new();

        let mut prev_range: Option<Range<u64>> = None;
        for range in ranges {
            if let Some(prev) = prev_range {
                if prev.end >= range.start {
                    // Overlap
                    let new_range = prev.start..range.end;
                    prev_range = Some(new_range);
                } else {
                    // No overlap
                    new_ranges.push(prev);
                    prev_range = Some(range);
                }
            } else {
                prev_range = Some(range);
            }
        }

        if let Some(prev) = prev_range {
            new_ranges.push(prev);
        }

        Self { ranges: new_ranges }
    }

    pub fn from_joined_ranges(ranges: Vec<ValueRanges>) -> Self {
        let ranges = ranges
            .into_iter()
            .flat_map(|ranges| ranges.ranges.into_iter())
            .to_vec();

        Self::new(ranges)
    }

    // Returns: old, new
    fn splice_ranges(&self, at: Range<u64>, new_start: u64) -> (ValueRanges, ValueRanges) {
        let mut old_ranges = Vec::new();
        let mut new_ranges = Vec::new();

        for range in self.ranges.iter() {
            let (old, new) = split_range_at(range.clone(), at.clone(), new_start);
            old_ranges.extend(old);
            if let Some(new) = new {
                new_ranges.push(new);
            }
        }

        (ValueRanges::new(old_ranges), ValueRanges::new(new_ranges))
    }
}

fn parse_line_to_map(line: &str) -> MapRange {
    let mut parts = line.split_whitespace();

    let dest_range = parts.next().unwrap().parse::<u64>().unwrap();
    let start_range = parts.next().unwrap().parse::<u64>().unwrap();
    let len = parts.next().unwrap().parse::<u64>().unwrap();

    MapRange {
        start_range: start_range..(start_range + len),
        dest_range: dest_range..(dest_range + len),
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map_number(&self, num: u64) -> u64 {
        // Find a range that matches
        let range = self
            .ranges
            .iter()
            .find(|range| range.start_range.contains(&num));

        if let Some(range) = range {
            let offset = num - range.start_range.start;
            let dest = range.dest_range.start + offset;
            dest
        } else {
            num
        }
    }

    fn map_number_range(&self, ranges: ValueRanges) -> ValueRanges {
        let mut remaining_ranges = ranges;
        let mut new_ranges = Vec::new();

        for map_range in &self.ranges {
            let (old, new) = remaining_ranges
                .splice_ranges(map_range.start_range.clone(), map_range.dest_range.start);
            new_ranges.push(new);
            remaining_ranges = old;
        }

        new_ranges.push(remaining_ranges);

        ValueRanges::from_joined_ranges(new_ranges)
    }
}

fn parse_map(map_block: &str) -> Map {
    let mut lines = map_block.lines();

    let name = lines.next().unwrap();
    let (name, _) = name.split_at_char(' ');
    let name = name.to_string();

    let mut ranges = Vec::new();

    for line in lines {
        ranges.push(parse_line_to_map(line));
    }

    Map { name, ranges }
}

#[derive(Debug)]
struct Maps {
    maps: Vec<Map>,
}

impl Maps {
    pub fn map_number(&self, num: u64) -> u64 {
        let mut num = num;
        for map in &self.maps {
            num = map.map_number(num);
        }
        num
    }

    pub fn map_number_range(&self, ranges: ValueRanges) -> ValueRanges {
        let mut ranges = ranges;
        for map in &self.maps {
            dbg!(&ranges);
            dbg!(&map.name);
            ranges = map.map_number_range(ranges);
        }
        ranges
    }
}

#[derive(Debug)]
struct InputData {
    seeds: Vec<u64>,
    maps: Maps,
}

fn parse_input(input: &str) -> InputData {
    let mut blocks = input.split("\n\n");

    let seeds_block = blocks.next().unwrap();
    let (_, seeds_str) = seeds_block.split_at_str(": ");
    let seeds = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = blocks.map(parse_map).collect::<Vec<_>>();

    InputData {
        seeds,
        maps: Maps { maps },
    }
}

fn part1() {
    let input = parse_input(INPUT);

    let locations = input.seeds.iter().map(|seed| input.maps.map_number(*seed));
    let min = locations.min().unwrap();

    println!("Part 1: {}", min);
}

fn part2() {
    let input = parse_input(INPUT);

    let value_ranges_vec = input
        .seeds
        .chunks(2)
        .map(|window| {
            let start = window[0];
            let len = window[1];
            start..(start + len)
        })
        .to_vec();

    let value_ranges = ValueRanges::new(value_ranges_vec);

    dbg!(&value_ranges.ranges);
    let value_ranges = input.maps.map_number_range(value_ranges);
    let min = value_ranges.ranges.iter().map(|r| r.start).min().unwrap();

    dbg!(&value_ranges.ranges);

    println!("Part 2: {}", min)
}

fn main() {
    part1();
    part2();
}
