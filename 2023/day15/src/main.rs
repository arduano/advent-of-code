use shared::*;

const INPUT: &str = day_input!();

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.

fn parse_input(input: &str) -> Vec<String> {
    // Split by comma, ignore newlines
    input.split(",").map(|s| s.trim().to_string()).collect()
}

// kf=9,gmn=6,sfvcm=2,cm-,dqb-,vxt=6,mnvm-,jd=7

#[derive(Debug, Clone)]
enum Instruction {
    Add { label: String, value: u32 },
    Remove { label: String },
}

fn hash_str(str: &str) -> u32 {
    let mut hash = 0;
    for c in str.chars() {
        hash += c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[derive(Debug)]
struct HashMap2 {
    // 256 long
    map: Vec<Vec<(String, u32)>>,
}

impl HashMap2 {
    fn new() -> Self {
        Self {
            map: vec![Vec::new(); 256],
        }
    }

    fn add(&mut self, label: String, value: u32) {
        let hash = hash_str(&label) as usize;
        // Find index
        if let Some((i, _)) = self.map[hash]
            .iter()
            .enumerate()
            .find(|(_, (l, _))| l == &label)
        {
            // Update
            self.map[hash][i] = (label, value);
        } else {
            // Insert
            self.map[hash].push((label, value));
        }
    }

    fn remove(&mut self, label: &str) {
        let hash = hash_str(label) as usize;
        // Find index
        if let Some((i, _)) = self.map[hash]
            .iter()
            .enumerate()
            .find(|(_, (l, _))| l == label)
        {
            // Remove
            self.map[hash].remove(i);
        }
    }
}

fn parse_str_to_instruction(str: &str) -> Instruction {
    if str.contains("=") {
        let mut split = str.split("=");
        let label = split.next().unwrap().to_string();
        let value = split.next().unwrap().parse::<u32>().unwrap();
        Instruction::Add { label, value }
    } else {
        Instruction::Remove {
            label: str[..str.len() - 1].to_string(),
        }
    }
}

fn part1() {
    let input = parse_input(INPUT);

    let sum = input.iter().map(|s| hash_str(s)).sum::<u32>();

    println!("Part 1: {}", sum)
}

fn part2() {
    let input = parse_input(INPUT);

    let instructions = input.iter().map(|s| parse_str_to_instruction(s)).to_vec();

    let mut map = HashMap2::new();
    for instruction in instructions {
        match instruction {
            Instruction::Add { label, value } => map.add(label, value),
            Instruction::Remove { label } => map.remove(&label),
        }
    }

    let mut sum = 0;
    for i in 0..256 {
        let slot = &map.map[i];
        for (j, (_, value)) in slot.iter().enumerate() {
            sum += value * (i + 1) as u32 * (j + 1) as u32;
        }
    }

    println!("Part 2: {}", sum)
}

fn main() {
    part1();
    part2();
}
