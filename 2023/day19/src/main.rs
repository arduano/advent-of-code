use std::{collections::VecDeque, ops::Range};

use shared::*;

const INPUT: &str = day_input!();

// px{a<2006:qkq,m>2090:A,rfg}
// pv{a>1716:R,A}
// lnx{m>1548:A,A}
// rfg{s<537:gd,x>2440:R,A}
// qs{s>3448:A,lnx}
// qkq{x<1416:A,crn}
// crn{x>2662:A,R}
// in{s<1351:px,qqz}
// qqz{s>2770:qs,m<1801:hdj,R}
// gd{a>3333:R,R}
// hdj{m>838:A,pv}

// {x=787,m=2655,a=1222,s=2876}
// {x=1679,m=44,a=2067,s=496}
// {x=2036,m=264,a=79,s=2244}
// {x=2461,m=1339,a=466,s=291}
// {x=2127,m=1623,a=2188,s=1013}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Property {
    X,
    M,
    A,
    S,
}

fn property_from_str(s: &str) -> Property {
    match s {
        "x" => Property::X,
        "m" => Property::M,
        "a" => Property::A,
        "s" => Property::S,
        _ => panic!("Unknown property {}", s),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn get_property(&self, property: Property) -> i32 {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }

    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

fn parse_part_from_str(s: &str) -> Part {
    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    let s = s.trim_start_matches('{').trim_end_matches('}');

    for prop in s.split(',') {
        let mut prop = prop.split('=');
        let name = prop.next().unwrap();
        let value = prop.next().unwrap().parse::<i32>().unwrap();
        match property_from_str(name) {
            Property::X => part.x = value,
            Property::M => part.m = value,
            Property::A => part.a = value,
            Property::S => part.s = value,
        }
    }
    part
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    rule_parts: Vec<RulePart>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RuleOp {
    Always,
    Lt,
    Gt,
}

impl RuleOp {
    fn check(&self, value: i32, other: i32) -> bool {
        match self {
            RuleOp::Always => true,
            RuleOp::Lt => value < other,
            RuleOp::Gt => value > other,
        }
    }
}

fn parse_rule_op_from_str(s: &str) -> RuleOp {
    match s {
        "<" => RuleOp::Lt,
        ">" => RuleOp::Gt,
        _ => panic!("Unknown rule op {}", s),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RulePart {
    property: Property,
    op: RuleOp,
    value: i32,
    rule_action: RuleAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RuleAction {
    GoToRule(String),
    Accept,
    Reject,
}

fn parse_rule_action_from_str(s: &str) -> RuleAction {
    match s {
        "R" => RuleAction::Reject,
        "A" => RuleAction::Accept,
        _ => RuleAction::GoToRule(s.to_string()),
    }
}

fn parse_rule_part_from_str(s: &str) -> RulePart {
    if !s.contains(':') {
        return RulePart {
            property: Property::X,
            op: RuleOp::Always,
            value: 0,
            rule_action: parse_rule_action_from_str(s),
        };
    }

    let (check, action) = s.split_at_char(':');
    let property = property_from_str(&check[0..1]);
    let op = parse_rule_op_from_str(&check[1..2]);
    let value = check[2..].parse::<i32>().unwrap();

    RulePart {
        property,
        op,
        value,
        rule_action: parse_rule_action_from_str(action),
    }
}

fn parse_rule_from_str(s: &str) -> Rule {
    let s = s.trim_end_matches('}');
    let (name, parts) = s.split_at_char('{');

    let mut rule_parts = Vec::new();
    for part in parts.split(',') {
        rule_parts.push(parse_rule_part_from_str(part));
    }

    Rule {
        name: name.to_string(),
        rule_parts,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InputData {
    rules: Vec<Rule>,
    parts: Vec<Part>,
}

fn parse_input() -> InputData {
    let mut rules = Vec::new();
    let mut parts = Vec::new();

    for line in INPUT.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains('=') {
            parts.push(parse_part_from_str(line));
        } else {
            rules.push(parse_rule_from_str(line));
        }
    }

    InputData { rules, parts }
}

fn part1() {
    let input = parse_input();

    let mut total_sum = 0;

    'outer: for part in input.parts {
        let mut current_rule = "in".to_string();

        'inner: loop {
            let rule = input.rules.iter().find(|r| r.name == current_rule).unwrap();

            for rule_part in &rule.rule_parts {
                let value = part.get_property(rule_part.property);

                if rule_part.op.check(value, rule_part.value) {
                    match rule_part.rule_action {
                        RuleAction::GoToRule(ref name) => {
                            current_rule = name.clone();
                            continue 'inner;
                        }
                        RuleAction::Accept => {
                            break 'inner;
                        }
                        RuleAction::Reject => {
                            continue 'outer;
                        }
                    }
                }
            }
        }

        total_sum += part.sum();
    }

    println!("Part 1: {}", total_sum)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PartRanges {
    x_range: Range<i32>,
    m_range: Range<i32>,
    a_range: Range<i32>,
    s_range: Range<i32>,
}

impl PartRanges {
    fn get_field_mut(&mut self, property: Property) -> &mut Range<i32> {
        match property {
            Property::X => &mut self.x_range,
            Property::M => &mut self.m_range,
            Property::A => &mut self.a_range,
            Property::S => &mut self.s_range,
        }
    }

    fn get_field(&self, property: Property) -> &Range<i32> {
        match property {
            Property::X => &self.x_range,
            Property::M => &self.m_range,
            Property::A => &self.a_range,
            Property::S => &self.s_range,
        }
    }

    fn product(&self) -> i64 {
        let mut product = 1;
        product *= self.x_range.end as i64 - self.x_range.start as i64;
        product *= self.m_range.end as i64 - self.m_range.start as i64;
        product *= self.a_range.end as i64 - self.a_range.start as i64;
        product *= self.s_range.end as i64 - self.s_range.start as i64;
        product
    }
}

// Returns: (accepted, rejected)
fn slice_range_by(ranges: &PartRanges, by: &RulePart) -> (Option<PartRanges>, Option<PartRanges>) {
    let mut accepted_cloned = ranges.clone();
    let mut rejected_cloned = ranges.clone();

    let accepted_field = accepted_cloned.get_field_mut(by.property);
    let rejected_field = rejected_cloned.get_field_mut(by.property);
    let field = ranges.get_field(by.property);

    let by_val = by.value;

    match by.op {
        RuleOp::Lt => {
            if field.start >= by_val {
                (None, Some(rejected_cloned))
            } else if field.end < by_val {
                (Some(accepted_cloned), None)
            } else {
                accepted_field.end = by_val;
                rejected_field.start = by_val;
                (Some(accepted_cloned), Some(rejected_cloned))
            }
        }
        RuleOp::Gt => {
            if field.end <= by_val + 1 {
                (None, Some(rejected_cloned))
            } else if field.start > by_val {
                (Some(accepted_cloned), None)
            } else {
                accepted_field.start = by_val + 1;
                rejected_field.end = by_val + 1;
                (Some(accepted_cloned), Some(rejected_cloned))
            }
        }
        RuleOp::Always => (Some(accepted_cloned), None),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchNode {
    rule: String,
    ranges: PartRanges,
}

fn part2() {
    let input = parse_input();

    let starting_ranges = PartRanges {
        x_range: 1..4001,
        m_range: 1..4001,
        a_range: 1..4001,
        s_range: 1..4001,
    };

    let start_node = SearchNode {
        rule: "in".to_string(),
        ranges: starting_ranges.clone(),
    };

    let mut queue = VecDeque::new();
    queue.push_back(start_node);

    let mut accepted_parts = Vec::new();

    while let Some(node) = queue.pop_front() {
        let rule = input.rules.iter().find(|r| r.name == node.rule).unwrap();

        let mut remaining_ranges = node.ranges;

        for rule_part in &rule.rule_parts {
            let (accepted_ranges, rejected_ranges) = slice_range_by(&remaining_ranges, rule_part);

            println!(
                "{:?}\n{:?}\n{:?}",
                rule_part, accepted_ranges, rejected_ranges
            );

            if let Some(accepted_ranges) = accepted_ranges {
                match rule_part.rule_action {
                    RuleAction::GoToRule(ref name) => {
                        let new_node = SearchNode {
                            rule: name.clone(),
                            ranges: accepted_ranges,
                        };
                        queue.push_back(new_node);
                    }
                    RuleAction::Accept => {
                        accepted_parts.push(accepted_ranges);
                    }
                    RuleAction::Reject => {}
                }
            }

            if let Some(rejected_ranges) = rejected_ranges {
                remaining_ranges = rejected_ranges;
            } else {
                break;
            }
        }
    }

    let product = accepted_parts.iter().map(|p| p.product()).sum::<i64>();

    println!("Part 2: {}", product);
}

fn main() {
    part1();
    part2();
}
