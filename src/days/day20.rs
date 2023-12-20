use std::collections::{HashMap, VecDeque};

use aoc2023::get_input;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug, Clone, Copy)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Module {
    modtype: ModuleType,
    onoff: bool,
    pulses: Vec<PulseType>,
    conjunction_last_pulses: HashMap<String, PulseType>,
    destinations: Vec<String>,
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let re = Regex::new(r"^([%&])?(\w+) -> (.+[, ]?)+$").unwrap();
    let mut modules: HashMap<String, Module> = input
        .trim()
        .split("\n")
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let name = caps.get(2).unwrap().as_str();
            let modtype = match caps.get(1).or_else(|| caps.get(2)).unwrap().as_str() {
                "%" => ModuleType::FlipFlop,
                "broadcaster" => ModuleType::Broadcaster,
                "&" => ModuleType::Conjunction,
                _ => panic!("unknown module type"),
            };
            let onoff = if matches!(modtype, ModuleType::FlipFlop) {
                false
            } else {
                true
            };

            (
                name.to_string(),
                Module {
                    modtype,
                    onoff,
                    pulses: vec![],
                    conjunction_last_pulses: HashMap::new(),
                    destinations: caps
                        .get(3)
                        .unwrap()
                        .as_str()
                        .split(",")
                        .map(|d| d.trim().to_string())
                        .collect(),
                },
            )
        })
        .collect();

    // fill conjuction modules with "low"
    let conjuctions: Vec<String> = modules
        .iter()
        .filter(|m| matches!(m.1.modtype, ModuleType::Conjunction))
        .map(|v| String::from(v.0))
        .collect();
    for conjuction in conjuctions {
        for m in modules
            .iter()
            .filter(|m| m.0 != &conjuction && m.1.destinations.contains(&conjuction))
            .map(|v| String::from(v.0))
            .collect::<Vec<String>>()
        {
            modules.entry(conjuction.clone()).and_modify(|e| {
                e.conjunction_last_pulses.insert(m, PulseType::Low);
            });
        }
    }
    modules
}

fn part1(input: HashMap<String, Module>) -> usize {
    // println!("{input:#?}");

    let mut current_state = input.clone();
    let mut module_queue = VecDeque::new();
    let mut sent_pulses = vec![];

    let mut button_pushes = 1000;
    while button_pushes > 0 {
        current_state
            .entry(String::from("broadcaster"))
            .and_modify(|e| e.pulses.push(PulseType::Low));
        sent_pulses.push(PulseType::Low);
        module_queue.push_back(String::from("broadcaster"));

        while let Some(current_module_name) = module_queue.pop_front() {
            // println!("---> {current_module_name}");
            // println!("{current_state:#?}");

            let current_module = current_state.get(&current_module_name).unwrap().clone();

            for destination in current_module.destinations.iter() {
                // if let Some(pulse) = current_module.pulses.pop() {
                for pulse in current_module.pulses.iter() {
                    sent_pulses.push(*pulse);

                    // println!("\n\n{current_module_name} -{pulse:?}->{destination}");
                    // println!("{current_state:?}");

                    current_state
                        .entry(destination.clone())
                        .and_modify(|e| match e.modtype {
                            ModuleType::FlipFlop => {
                                if matches!(pulse, PulseType::Low) {
                                    if e.onoff {
                                        e.pulses.push(PulseType::Low);
                                    } else {
                                        e.pulses.push(PulseType::High);
                                    }

                                    e.onoff = !e.onoff;
                                    module_queue.push_back(String::from(destination));
                                }
                            }
                            ModuleType::Conjunction => {
                                e.conjunction_last_pulses
                                    .insert(current_module_name.to_string(), pulse.clone());
                                let last_pulses = e
                                    .conjunction_last_pulses
                                    .values()
                                    .map(|v| *v)
                                    .collect::<Vec<PulseType>>();
                                // println!("last_pulses {last_pulses:?}");
                                if last_pulses.iter().all(|p| matches!(p, PulseType::High)) {
                                    e.pulses.push(PulseType::Low);
                                } else {
                                    e.pulses.push(PulseType::High);
                                }
                                module_queue.push_back(String::from(destination));
                            }
                            ModuleType::Broadcaster => {
                                e.pulses.push(*pulse);
                                module_queue.push_back(String::from(destination));
                            }
                        });
                }
            }
            current_state
                .entry(current_module_name)
                .and_modify(|e| e.pulses = vec![]);
        }
        button_pushes -= 1;
    }

    // println!("{current_state:#?}");

    let low = sent_pulses
        .iter()
        .filter(|p| matches!(p, PulseType::Low))
        .count();
    let high = sent_pulses
        .iter()
        .filter(|p| matches!(p, PulseType::High))
        .count();

    low * high
}
// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
    ";
    const EXAMPLE2: &str = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 32000000);
        assert_eq!(part1(parse_input(EXAMPLE2)), 11687500);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 944750144);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
