use aoc2023::get_input;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    operation: char,
    box_number: u32,
    length: Option<u32>,
}

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let input: Vec<&str> = input.split(",").collect();
    input.iter().map(|v| hash(v)).sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(?P<label>[a-z]+)(?P<operation>=|-)(?P<length>\d+)?").unwrap();
    let lenses: Vec<Lens> = input
        .split(",")
        .map(|lens| {
            let cap = re.captures(lens).unwrap();
            let label = cap["label"].to_string();
            Lens {
                label: label.clone(),
                box_number: hash(&label),
                operation: cap["operation"].parse().unwrap(),
                length: cap.name("length").map(|v| v.as_str().parse().unwrap()),
            }
        })
        .collect();

    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();

    for lens in lenses {
        match lens.operation {
            '-' => {
                boxes.entry(lens.box_number).and_modify(|b| {
                    b.retain(|l| l.label != lens.label);
                });
            }
            '=' => {
                boxes
                    .entry(lens.box_number)
                    .and_modify(|b| {
                        if let Some(already_in) = b.iter_mut().find(|x| x.label == lens.label) {
                            already_in.length = lens.length;
                        } else {
                            b.push(lens.clone());
                        }
                    })
                    .or_insert(vec![lens]);
            }
            _ => panic!("unknown operation"),
        };
    }

    let mut focusing_power = 0;
    for lenses in boxes.values() {
        for (i, lens) in lenses.iter().enumerate() {
            focusing_power += (lens.box_number + 1) * (i as u32 + 1) * lens.length.unwrap();
        }
    }

    focusing_power
}

fn hash(s: &str) -> u32 {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value = current_value.rem_euclid(256);
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1320);
        assert_eq!(part1(&get_input!(file!())), 509152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 145);
        assert_eq!(part2(&get_input!(file!())), 244403);
    }
}
