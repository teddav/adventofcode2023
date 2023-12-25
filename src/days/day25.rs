use std::collections::{HashMap, HashSet};

use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

fn part1(map: HashMap<&str, HashSet<&str>>) -> u32 {
    println!("{map:#?}");
    0
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map: HashMap<&str, HashSet<&str>> = input
        .trim()
        .split("\n")
        .map(|line| {
            let components: [&str; 2] = line.split(":").collect::<Vec<&str>>().try_into().unwrap();
            (components[0], components[1].trim().split(" ").collect())
        })
        .collect();
    for (comp, connected) in map.clone().iter() {
        for conn in connected {
            map.entry(&conn)
                .and_modify(|e| {
                    e.insert(comp);
                })
                .or_default()
                .insert(comp);
        }
    }
    map
}

// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 54);
        // assert_eq!(part1(parse_input(&get_input!(file!()))), 0);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
