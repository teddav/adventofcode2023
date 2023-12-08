use aoc2023::get_input;
use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, ops::Rem};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> (&str, HashMap<String, Node>) {
    let mut input = input.trim().split("\n\n");
    let directions = input.next().unwrap();

    let node_re = Regex::new(r"(?P<id>\w+) = \((?P<left>\w+), (?P<right>\w+)\)").unwrap();
    let nodes = HashMap::from_iter(
        input
            .next()
            .unwrap()
            .split("\n")
            .map(|node| {
                let captures = node_re.captures(node).unwrap();
                (
                    captures["id"].to_string(),
                    Node {
                        left: captures["left"].to_string(),
                        right: captures["right"].to_string(),
                    },
                )
            })
            .collect::<Vec<(String, Node)>>(),
    );
    (directions, nodes)
}

fn part1((directions, nodes): (&str, HashMap<String, Node>)) -> usize {
    let mut step = 0;
    let mut current_node = nodes.get("AAA").unwrap();
    loop {
        let direction = directions.chars().nth(step.rem(directions.len())).unwrap();
        step += 1;
        let next_node = match direction {
            'R' => &current_node.right,
            'L' => &current_node.left,
            _ => panic!("wrong direction"),
        };
        if next_node == "ZZZ" {
            break;
        }
        current_node = nodes.get(next_node).unwrap();
    }
    step
}
fn part2((directions, nodes): (&str, HashMap<String, Node>)) -> usize {
    let start_nodes: Vec<&Node> = nodes
        .iter()
        .filter(|(key, _)| key.chars().last().unwrap() == 'A')
        .map(|(_, node)| node)
        .collect();

    let steps: Vec<usize> = start_nodes
        .into_iter()
        .map(|mut current_node| {
            let mut step = 0;
            loop {
                let direction = directions.chars().nth(step.rem(directions.len())).unwrap();
                step += 1;
                let next_node = match direction {
                    'R' => &current_node.right,
                    'L' => &current_node.left,
                    _ => panic!("wrong direction"),
                };
                if next_node.chars().last().unwrap() == 'Z' {
                    break;
                }
                current_node = nodes.get(next_node).unwrap();
            }
            step
        })
        .collect();

    steps.iter().fold(1, |acc, n| lcm(acc, *n))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
    ";

    const EXAMPLE2: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
    ";

    const EXAMPLE3: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE1)), 2);
        assert_eq!(part1(parse_input(EXAMPLE2)), 6);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 16897);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE3)), 6);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 16563603485021);
    }
}
