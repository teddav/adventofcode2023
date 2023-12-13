use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .trim()
        .split("\n\n")
        .map(|pattern| pattern.split("\n").collect())
        .collect()
}

fn part1(input: Vec<Vec<&str>>) -> usize {
    input
        .iter()
        .map(|pattern| {
            if let Some(vertical) = pattern_symmetry(pattern) {
                return vertical;
            }
            if let Some(horizontal) = pattern_symmetry(
                &reverse_pattern(pattern)
                    .iter()
                    .map(|s| s.as_str())
                    .collect(),
            ) {
                return horizontal * 100;
            }

            0
        })
        .sum()
}

fn reverse_pattern(pattern: &Vec<&str>) -> Vec<String> {
    (0..pattern[0].len())
        .map(|i| {
            pattern
                .into_iter()
                .map(|line| line.as_bytes()[i] as char)
                .collect::<String>()
        })
        .collect()
}

fn pattern_symmetry(pattern: &Vec<&str>) -> Option<usize> {
    let mut symmetry: Option<Vec<usize>> = None;

    for line in pattern {
        let sym = line_symmetry(line);
        let mut _symmetry = symmetry.unwrap_or(sym.clone());
        _symmetry.retain(|v| sym.contains(v));
        symmetry = Some(_symmetry);
    }

    symmetry.and_then(|s| if s.len() > 0 { Some(s[0]) } else { None })
}

fn line_symmetry(line: &str) -> Vec<usize> {
    (1..line.len())
        .map(|i| {
            let bound = i.min(line.len() - i);
            let start = i - bound;
            let end = i + bound;

            let a = &line[start..i];
            let b = &line[i..end];

            if a == &b.chars().rev().collect::<String>() {
                Some(i)
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 405);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 34100);
    }
}
