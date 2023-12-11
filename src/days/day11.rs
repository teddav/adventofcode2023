use std::collections::HashMap;

use aoc2023::get_input;

#[derive(Debug)]
struct Position(usize, usize);

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input), 1000000));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|r| r.chars().collect())
        .collect()
}

fn map_expansion(
    map: Vec<Vec<char>>,
    multiplier: usize,
) -> (HashMap<u32, Position>, Vec<(u32, u32)>) {
    let empty_rows: Vec<usize> = map
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r))
        .filter(|r| r.1.iter().all(|&c| c == '.'))
        .map(|r| r.0)
        .collect();
    let mut empty_cols: Vec<usize> = vec![];
    for col in 0..map[0].len() {
        let column: Vec<char> = map.iter().map(|r| r[col]).collect();
        if column.into_iter().all(|c| c == '.') {
            empty_cols.push(col);
        }
    }

    let mut added_rows = 0;
    let mut added_cols = 0;

    let mut galaxies = HashMap::new();
    let mut galaxy_id = 1;
    for (y, row) in map.iter().enumerate() {
        if empty_rows.contains(&y) {
            added_rows += multiplier - 1;
        }
        for (x, col) in row.iter().enumerate() {
            if empty_cols.contains(&x) {
                added_cols += multiplier - 1;
            }
            if *col == '#' {
                galaxies.insert(galaxy_id, Position(x + added_cols, y + added_rows));
                galaxy_id += 1;
            }
        }
        added_cols = 0;
    }

    let mut pairs = vec![];
    for id1 in 1..galaxy_id {
        for id2 in id1 + 1..galaxy_id {
            pairs.push((id1, id2));
        }
    }

    (galaxies, pairs)
}

fn part1(map: Vec<Vec<char>>) -> usize {
    let (galaxies, pairs) = map_expansion(map, 2);
    pairs
        .into_iter()
        .map(|pair| {
            let start = galaxies.get(&pair.0).unwrap();
            let end = galaxies.get(&pair.1).unwrap();
            let distance = end.0.abs_diff(start.0) + end.1.abs_diff(start.1);
            distance
        })
        .sum()
}

fn part2(map: Vec<Vec<char>>, multiplier: usize) -> usize {
    let (galaxies, pairs) = map_expansion(map, multiplier);
    pairs
        .into_iter()
        .map(|pair| {
            let start = galaxies.get(&pair.0).unwrap();
            let end = galaxies.get(&pair.1).unwrap();
            let distance = end.0.abs_diff(start.0) + end.1.abs_diff(start.1);
            distance
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 374);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 9521550);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE), 10), 1030);
        assert_eq!(part2(parse_input(EXAMPLE), 100), 8410);
        assert_eq!(
            part2(parse_input(&get_input!(file!())), 1000000),
            298932923702
        );
    }
}
