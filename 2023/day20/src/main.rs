use std::collections::{HashMap, VecDeque};

use shared::*;

const INPUT: &str = day_input!();

// broadcaster -> a
// %a -> inv, con
// &inv -> b
// %b -> con
// &con -> output

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleKind {
    None,
    FlipFlop,    // %
    Conjunction, // &
}

#[derive(Debug)]
struct Rule {
    kind: RuleKind,
    input: String,
    outputs: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Rule> {
    let mut rules = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let input = parts.next().unwrap();
        let output = parts.next().unwrap();
        let mut outputs = Vec::new();
        for output in output.split(", ") {
            outputs.push(output.to_string());
        }
        let mut kind = RuleKind::None;
        let mut input = input.to_string();
        if input.starts_with("%") {
            kind = RuleKind::FlipFlop;
            input = input[1..].to_string();
        } else if input.starts_with("&") {
            kind = RuleKind::Conjunction;
            input = input[1..].to_string();
        }
        rules.push(Rule {
            kind,
            input,
            outputs,
        });
    }
    rules
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseMode {
    Low,
    High,
}

impl PulseMode {
    fn flip(self) -> Self {
        match self {
            PulseMode::Low => PulseMode::High,
            PulseMode::High => PulseMode::Low,
        }
    }
}

#[derive(Debug)]
struct Memory {
    state: State,
}

#[derive(Debug)]
struct State {
    conjunctions: HashMap<String, HashMap<String, PulseMode>>,
    flip_flops: HashMap<String, PulseMode>,
}

impl State {
    fn get_pulse_for_conjunction(
        &mut self,
        module: &str,
        input: &str,
        mode: PulseMode,
    ) -> PulseMode {
        let module = self.conjunctions.get_mut(module).unwrap();
        let input = module.get_mut(input).unwrap();

        // Update the input
        *input = mode;

        // If all inputs are high, return low. Otherwise return high.
        if module.values().all(|input| *input == PulseMode::High) {
            PulseMode::Low
        } else {
            PulseMode::High
        }
    }

    fn get_pulse_for_flip_flop(&mut self, module: &str, mode: PulseMode) -> Option<PulseMode> {
        let module = self.flip_flops.get_mut(module).unwrap();

        if mode == PulseMode::High {
            return None;
        }

        *module = module.flip();
        Some(*module)
    }
}

impl Memory {
    fn new(rules: &Vec<Rule>) -> Self {
        let conjunction_modules = rules
            .iter()
            .filter(|rule| rule.kind == RuleKind::Conjunction)
            .map(|rule| rule.input.clone())
            .collect::<Vec<_>>();

        let mut conjunction_map = HashMap::new();
        for module in conjunction_modules {
            let inputs = rules
                .iter()
                .filter(|rule| rule.outputs.contains(&module))
                .map(|rule| rule.input.clone())
                .collect::<Vec<_>>();

            let mut input_map = HashMap::new();
            for input in inputs {
                input_map.insert(input, PulseMode::Low);
            }

            conjunction_map.insert(module, input_map);
        }

        let flip_flop_modules = rules
            .iter()
            .filter(|rule| rule.kind == RuleKind::FlipFlop)
            .map(|rule| rule.input.clone())
            .collect::<Vec<_>>();

        let mut flip_flop_map = HashMap::new();
        for module in flip_flop_modules {
            flip_flop_map.insert(module, PulseMode::Low);
        }

        Self {
            state: State {
                conjunctions: conjunction_map,
                flip_flops: flip_flop_map,
            },
        }
    }

    fn get_results_for_signal<'a>(
        &mut self,
        rules: &'a [Rule],
        signal: Signal<'a>,
    ) -> Option<impl 'a + Iterator<Item = Signal<'a>>> {
        let rule = rules.iter().find(|rule| rule.input == signal.to);

        let Some(rule) = rule else {
            return None;
        };

        let make_signals = |mode: PulseMode| {
            Some(rule.outputs.iter().map(move |output| Signal {
                from: &signal.to,
                to: &output,
                mode,
            }))
        };

        match rule.kind {
            RuleKind::None => make_signals(signal.mode),
            RuleKind::FlipFlop => {
                let mode = self.state.get_pulse_for_flip_flop(&signal.to, signal.mode);
                if let Some(mode) = mode {
                    make_signals(mode)
                } else {
                    None
                }
            }
            RuleKind::Conjunction => {
                let mode =
                    self.state
                        .get_pulse_for_conjunction(&signal.to, &signal.from, signal.mode);
                make_signals(mode)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Signal<'a> {
    from: &'a str,
    to: &'a str,
    mode: PulseMode,
}

impl std::fmt::Display for Signal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self.mode {
            PulseMode::Low => "low",
            PulseMode::High => "high",
        };
        write!(f, "{} -{}-> {}", self.from, mode, self.to)
    }
}

fn part1() {
    let input = parse_input(INPUT);
    let mut memory = Memory::new(&input);

    let initial_signal = Signal {
        from: &"_",
        to: &"broadcaster",
        mode: PulseMode::Low,
    };

    let mut lows_sent = 0;
    let mut highs_sent = 0;

    for _ in 0..1000 {
        let mut signals = VecDeque::new();
        signals.push_back(initial_signal.clone());

        while let Some(signal) = signals.pop_front() {
            let results = memory.get_results_for_signal(&input, signal);
            let Some(results) = results else {
                continue;
            };

            for result in results {
                println!("{}", result);

                if result.mode == PulseMode::Low {
                    lows_sent += 1;
                } else {
                    highs_sent += 1;
                }

                signals.push_back(result);
            }
        }

        lows_sent += 1;
    }

    println!("Lows sent: {}", lows_sent);
    println!("Highs sent: {}", highs_sent);

    let mut product = lows_sent * highs_sent;

    println!("Part 1: {}", product)
}

fn part2() {
    let input = parse_input(INPUT);
    let mut memory = Memory::new(&input);

    let initial_signal = Signal {
        from: &"_",
        to: &"broadcaster",
        mode: PulseMode::Low,
    };

    let mut lows_sent = 0;
    let mut highs_sent = 0;

    let mut fewest = 0;

    'outer: for i in 0.. {
        if i % 10000 == 0 {
            println!("i: {}", i);
        }

        let mut signals = VecDeque::new();
        signals.push_back(initial_signal.clone());

        while let Some(signal) = signals.pop_front() {
            let results = memory.get_results_for_signal(&input, signal);
            let Some(results) = results else {
                continue;
            };

            for result in results {
                if result.mode == PulseMode::Low {
                    lows_sent += 1;
                } else {
                    highs_sent += 1;
                }

                if result.to == "rx" && result.mode == PulseMode::Low {
                    fewest = i;
                    break 'outer;
                }

                signals.push_back(result);
            }
        }

        lows_sent += 1;
    }

    println!("Part 2: {}", fewest)
}

fn main() {
    part1();
    part2();
}
