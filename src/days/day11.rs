use std::collections::HashMap;

use aoc2023::get_input;

#[derive(Debug)]
struct Position(usize, usize);

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<u32, Position>, Vec<(u32, u32)>) {
    let mut map: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|r| r.chars().collect())
        .collect();
    let empty_row: Vec<char> = ".".repeat(map[0].len()).chars().collect();
    let mut i = 0;
    while i < map.len() {
        if map[i].iter().all(|&c| c == '.') {
            map.insert(i, empty_row.clone());
            i += 1;
        }
        i += 1;
    }

    i = 0;
    while i < map[0].len() {
        let column: Vec<char> = map.iter().map(|r| r[i]).collect();
        if column.into_iter().all(|c| c == '.') {
            for j in 0..map.len() {
                map[j].insert(i, '.');
            }
            i += 1;
        }
        i += 1;
    }

    let mut galaxies = HashMap::new();
    let mut galaxy_id = 1;
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.insert(galaxy_id, Position(x, y));
                galaxy_id += 1;
            }
        }
    }

    let mut pairs = vec![];
    for id1 in 1..galaxy_id {
        for id2 in id1 + 1..galaxy_id {
            pairs.push((id1, id2));
        }
    }

    (map, galaxies, pairs)
}

fn part1((_, galaxies, pairs): (Vec<Vec<char>>, HashMap<u32, Position>, Vec<(u32, u32)>)) -> usize {
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

// fn part2(input: Vec<&str>) {}

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
        assert_eq!(part1(parse_input(&get_input!(file!()))), 0);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
