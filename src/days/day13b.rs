use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input
        .trim()
        .split("\n\n")
        .map(|pattern| pattern.split("\n").map(|s| s.to_string()).collect())
        .collect()
}

fn part2(input: Vec<Vec<String>>) -> usize {
    let mut symmetries = vec![];

    'main: for pattern in input {
        let vertical = pattern_symmetry(&pattern, 0).unwrap_or(0);
        let horizontal = pattern_symmetry(&reverse_pattern(&pattern), 0).unwrap_or(0);

        for (row, line) in pattern.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let mut _pattern = pattern.clone();
                let opposite = match c {
                    '.' => "#",
                    '#' => ".",
                    _ => panic!("unknown char"),
                };
                _pattern[row].replace_range(col..col + 1, opposite);

                if let Some(new_vertical) = pattern_symmetry(&_pattern, vertical) {
                    symmetries.push(new_vertical);
                    continue 'main;
                }

                if let Some(new_horizontal) =
                    pattern_symmetry(&reverse_pattern(&_pattern), horizontal)
                {
                    symmetries.push(new_horizontal * 100);
                    continue 'main;
                }
            }
        }
    }

    symmetries.iter().sum()
}

fn reverse_pattern(pattern: &Vec<String>) -> Vec<String> {
    (0..pattern[0].len())
        .map(|i| {
            pattern
                .iter()
                .map(|line| line.as_bytes()[i] as char)
                .collect::<String>()
        })
        .collect()
}

fn pattern_symmetry(pattern: &Vec<String>, old_reflection: usize) -> Option<usize> {
    let mut symmetry: Option<Vec<usize>> = None;

    for line in pattern {
        let sym = line_symmetry(line);
        let mut _symmetry = symmetry.unwrap_or(sym.clone());
        _symmetry.retain(|v| sym.contains(v) && *v != old_reflection);
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
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 400);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 33106);
    }
}
