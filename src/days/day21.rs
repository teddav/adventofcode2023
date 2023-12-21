use aoc2023::get_input;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input), 64));
    println!("Part2: {}", part2(parse_input(&input), 26501365));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(usize, usize);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Pos) {
    let mut start = Pos(0, 0);
    let input = input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Pos(x, y);
                        '.'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();
    (input, start)
}

fn part1((map, start): (Vec<Vec<char>>, Pos), max_steps: usize) -> usize {
    let mut positions = VecDeque::from([(start, max_steps)]);
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut answer: HashSet<Pos> = HashSet::new();

    while let Some((current_pos, steps)) = positions.pop_front() {
        if steps % 2 == 0 {
            answer.insert(current_pos);
        }
        if steps == 0 {
            continue;
        }

        for next_pos in next_positions(&map, &current_pos) {
            if seen.contains(&next_pos) {
                continue;
            }
            seen.insert(next_pos);
            positions.push_back((next_pos, steps - 1));
        }
    }
    answer.len()
}

fn next_positions(map: &Vec<Vec<char>>, pos: &Pos) -> Vec<Pos> {
    let mut next = vec![];
    if pos.0 > 0 {
        let pos = Pos(pos.0 - 1, pos.1);
        if map[pos.1][pos.0] == '.' {
            next.push(pos);
        }
    }
    if pos.1 > 0 {
        let pos = Pos(pos.0, pos.1 - 1);
        if map[pos.1][pos.0] == '.' {
            next.push(pos);
        }
    }
    if pos.0 < map[0].len() - 1 {
        let pos = Pos(pos.0 + 1, pos.1);
        if map[pos.1][pos.0] == '.' {
            next.push(pos);
        }
    }
    if pos.1 < map.len() - 1 {
        let pos = Pos(pos.0, pos.1 + 1);
        if map[pos.1][pos.0] == '.' {
            next.push(pos);
        }
    }
    next
}

fn part2((map, start): (Vec<Vec<char>>, Pos), max_steps: usize) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE), 6), 16);
        assert_eq!(part1(parse_input(EXAMPLE), 64), 42);
        assert_eq!(part1(parse_input(&get_input!(file!())), 64), 3666);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE), 6), 16);
        assert_eq!(part2(parse_input(EXAMPLE), 10), 50);
        assert_eq!(part2(parse_input(EXAMPLE), 50), 1594);
        assert_eq!(part2(parse_input(EXAMPLE), 100), 6536);
        assert_eq!(part2(parse_input(EXAMPLE), 500), 167004);
        assert_eq!(part2(parse_input(EXAMPLE), 1000), 668697);
        assert_eq!(part2(parse_input(EXAMPLE), 5000), 16733044);
        assert_eq!(part2(parse_input(&get_input!(file!())), 26501365), 0);
    }
}
