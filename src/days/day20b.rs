use aoc2023::get_input;
use num::integer::lcm;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part2: {}", part2(parse_input(&input)));
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
    name: String,
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
                    name: name.to_string(),
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

fn part2(input: HashMap<String, Module>) -> usize {
    let parent = input
        .iter()
        .filter(|m| m.1.destinations.contains(&String::from("rx")))
        .map(|m| m.1.clone())
        .collect::<Vec<Module>>()[0]
        .clone();
    let mut parents: Vec<(String, usize)> = parent
        .conjunction_last_pulses
        .keys()
        .map(|v| (String::from(v), 0))
        .collect();

    let mut current_state = input.clone();
    let mut module_queue = VecDeque::new();
    let mut button_pushes = 1;

    loop {
        if parents.iter().all(|p| p.1 > 0) {
            break;
        }

        current_state
            .entry(String::from("broadcaster"))
            .and_modify(|e| e.pulses.push(PulseType::Low));
        module_queue.push_back(String::from("broadcaster"));

        while let Some(current_module_name) = module_queue.pop_front() {
            let current_module = current_state.get(&current_module_name).unwrap().clone();

            for destination in current_module.destinations.iter() {
                for pulse in current_module.pulses.iter() {
                    if destination == &parent.name && matches!(pulse, PulseType::High) {
                        if let Some(i) = parents.iter().position(|p| p.0 == current_module_name) {
                            if parents[i].1 == 0 {
                                parents[i].1 = button_pushes;
                            }
                        }
                    }

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
        button_pushes += 1;
    }

    parents.iter().fold(1, |acc, n| lcm(acc, n.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(&get_input!(file!()))), 222718819437131);
    }
}
