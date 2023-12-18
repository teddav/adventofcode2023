use aoc2023::get_input;
use regex::Regex;

#[derive(Debug)]
struct Dig {
    direction: char,
    length: i64,
    color: String,
}

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<Dig> {
    let re = Regex::new(r"(?P<dir>\w) (?P<len>\d+) \(#(?P<color>\w+)\)").unwrap();
    input
        .trim()
        .split("\n")
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Dig {
                direction: cap["dir"].as_bytes()[0] as char,
                length: cap["len"].parse().unwrap(),
                color: cap["color"].to_string(),
            }
        })
        .collect()
}

fn part1(digs: Vec<Dig>) -> i64 {
    let mut path = vec![(0, 0)];

    for dig in digs {
        let mut current = path.last().unwrap().clone();
        current = match dig.direction {
            'L' => (current.0 - dig.length, current.1),
            'R' => (current.0 + dig.length, current.1),
            'U' => (current.0, current.1 - dig.length),
            'D' => (current.0, current.1 + dig.length),
            _ => panic!("unknown dir"),
        };
        path.push(current);
    }

    let outter_cubes: i64 = path
        .windows(2)
        .map(|points| (points[0].0 - points[1].0).abs() + (points[0].1 - points[1].1).abs())
        .sum();

    let area = shoelace(&path);

    // https://fr.wikipedia.org/wiki/Th%C3%A9or%C3%A8me_de_Pick
    let inner_cubes = area + 1 - outter_cubes as i64 / 2;

    inner_cubes + outter_cubes
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn shoelace(edges: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    for i in 1..edges.len() - 1 {
        sum += edges[i].0 * (edges[i + 1].1 - edges[i - 1].1);
    }
    sum.abs() / 2
}

fn part2(digs: Vec<Dig>) -> i64 {
    let digs: Vec<Dig> = digs
        .into_iter()
        .map(|dig| {
            let length = i64::from_str_radix(&dig.color[..5], 16).unwrap();
            let direction = match dig.color.as_bytes()[5] - ('0' as u8) {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => panic!("unknown dir"),
            };
            Dig {
                direction,
                length,
                color: dig.color,
            }
        })
        .collect();
    part1(digs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 62);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 40131);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 952408144115);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 104454050898331);
    }
}
