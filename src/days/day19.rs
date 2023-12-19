use std::collections::HashMap;

use aoc2023::get_input;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug, Clone, Copy)]
struct Condition(char, char, u32);
impl Condition {
    fn from(s: &str) -> Self {
        let re_condition = Regex::new(r"^([xmas])([<>])(\d+)$").unwrap();
        let condition = re_condition.captures(s).unwrap();
        Self(
            condition.get(1).unwrap().as_str().as_bytes()[0] as char,
            condition.get(2).unwrap().as_str().as_bytes()[0] as char,
            condition.get(3).unwrap().as_str().parse().unwrap(),
        )
    }

    fn matches(&self, part: &Part) -> bool {
        let value = match self.0 {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("unknown condition"),
        };
        match self.1 {
            '<' => value < self.2,
            '>' => value > self.2,
            _ => panic!("unknown condition 2"),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    destination: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

type Workflows = HashMap<String, Workflow>;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_input(input: &str) -> (Workflows, Vec<Part>) {
    let mut split = input.trim().split("\n\n");

    let re_workflow = Regex::new(r"^(?<name>\w+)\{(?:(?<rules>.+),?)+\}$").unwrap();
    let workflows = split
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let caps = re_workflow.captures(line).unwrap();
            let rules = caps["rules"].split(",").collect::<Vec<&str>>();

            (
                caps["name"].to_string(),
                Workflow {
                    name: caps["name"].to_string(),
                    rules: rules
                        .into_iter()
                        .map(|r| {
                            let rule = r.split(":").collect::<Vec<&str>>();
                            if rule.len() == 1 {
                                Rule {
                                    condition: None,
                                    destination: rule[0].to_string(),
                                }
                            } else {
                                Rule {
                                    condition: Some(Condition::from(rule[0])),
                                    destination: rule[1].to_string(),
                                }
                            }
                        })
                        .collect(),
                },
            )
        })
        .collect::<Workflows>();

    let re_part = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let parts = split
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let caps = re_part.captures(line).unwrap();
            Part {
                x: caps.get(1).unwrap().as_str().parse().unwrap(),
                m: caps.get(2).unwrap().as_str().parse().unwrap(),
                a: caps.get(3).unwrap().as_str().parse().unwrap(),
                s: caps.get(4).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect::<Vec<Part>>();

    (workflows, parts)
}

fn part1((workflows, parts): (Workflows, Vec<Part>)) -> u32 {
    let mut accepted = vec![];

    'main: for part in parts {
        let mut current_workflow = workflows.get("in").unwrap();

        loop {
            for rule in current_workflow.rules.iter() {
                let destination = if let Some(condition) = rule.condition {
                    if condition.matches(&part) {
                        Some(&rule.destination)
                    } else {
                        None
                    }
                } else {
                    Some(&rule.destination)
                };

                if let Some(destination) = destination {
                    if is_final(destination) {
                        if destination == "A" {
                            accepted.push(part);
                        }
                        continue 'main;
                    } else {
                        current_workflow = workflows.get(destination).unwrap();
                        break;
                    }
                }
            }
        }
    }

    accepted
        .iter()
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn is_final(destination: &str) -> bool {
    destination == "A" || destination == "R"
}

// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 19114);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 319295);
        // 241508
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
