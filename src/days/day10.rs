use aoc2023::get_input;
use std::collections::{HashSet, VecDeque};

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(usize, usize);
impl Position {
    fn next(&self, map: &Map) -> Vec<Position> {
        let max = (map.len(), map[0].len());
        let mut _next = vec![];
        match map[self.1][self.0] {
            'S' => {
                if self.0 > 0 {
                    if let Some(&left) = map.get(self.1).and_then(|line| line.get(self.0 - 1)) {
                        if left == '-' || left == 'L' || left == 'F' {
                            _next.push(Position(self.0 - 1, self.1));
                        }
                    }
                }
                if self.1 > 0 {
                    if let Some(&up) = map.get(self.1 - 1).and_then(|line| line.get(self.0)) {
                        if up == '|' || up == '7' || up == 'F' {
                            _next.push(Position(self.0, self.1 - 1));
                        }
                    }
                }
                if let Some(&right) = map.get(self.1).and_then(|line| line.get(self.0 + 1)) {
                    if right == '-' || right == '7' || right == 'J' {
                        _next.push(Position(self.0 + 1, self.1));
                    }
                }
                if let Some(&down) = map.get(self.1 + 1).and_then(|line| line.get(self.0)) {
                    if down == '|' || down == 'L' || down == 'J' {
                        _next.push(Position(self.0, self.1 + 1));
                    }
                }
            }
            '|' => {
                if self.1 > 0 {
                    _next.push(Position(self.0, self.1 - 1));
                }
                if self.1 < max.1 {
                    _next.push(Position(self.0, self.1 + 1));
                }
            }
            '-' => {
                if self.0 > 0 {
                    _next.push(Position(self.0 - 1, self.1));
                }
                if self.0 < max.0 {
                    _next.push(Position(self.0 + 1, self.1));
                }
            }
            'L' => {
                if self.1 > 0 {
                    _next.push(Position(self.0, self.1 - 1));
                }
                if self.0 < max.0 {
                    _next.push(Position(self.0 + 1, self.1));
                }
            }
            'J' => {
                if self.1 > 0 {
                    _next.push(Position(self.0, self.1 - 1));
                }
                if self.0 > 0 {
                    _next.push(Position(self.0 - 1, self.1));
                }
            }
            '7' => {
                if self.1 < max.1 {
                    _next.push(Position(self.0, self.1 + 1));
                }
                if self.0 > 0 {
                    _next.push(Position(self.0 - 1, self.1));
                }
            }
            'F' => {
                if self.0 < max.0 {
                    _next.push(Position(self.0 + 1, self.1));
                }
                if self.1 < max.1 {
                    _next.push(Position(self.0, self.1 + 1));
                }
            }
            '.' => panic!("you're in the wrong place"),
            _ => panic!("unknown pipe {}", map[self.1][self.0]),
        }

        // _next.into_iter().filter(|s| map[s.1][s.0] != '.').collect()
        _next
    }
}

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> (Map, Position) {
    let mut start = Position(0, 0);

    let map = input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Position(x, y);
                    }
                    c
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn part1((map, start): (Map, Position)) -> u32 {
    let mut positions = VecDeque::from_iter(vec![(start, 0)]);
    let mut already_seen: HashSet<Position> = HashSet::new();

    let mut max = 0;

    while let Some((position, moves)) = positions.pop_front() {
        if moves > max {
            max = moves;
        }

        if already_seen.contains(&position) || map[position.1][position.0] == '.' {
            continue;
        }
        already_seen.insert(position);

        for next_position in position.next(&map) {
            if !already_seen.contains(&next_position) {
                positions.push_back((next_position, moves + 1));
            }
        }
    }
    max
}

fn part2((map, start): (Map, Position)) -> usize {
    let mut paths = VecDeque::from_iter(vec![vec![start]]);
    let mut already_seen: HashSet<Position> = HashSet::new();

    let mut max = 0;

    while let Some(path) = paths.pop_front() {
        let current_position = path.last().unwrap().to_owned();

        if path.len() - 1 > max {
            max = path.len() - 1;
        }

        if already_seen.contains(&current_position)
            || map[current_position.1][current_position.0] == '.'
        {
            continue;
        }
        already_seen.insert(current_position);

        for next_position in current_position.next(&map) {
            if !already_seen.contains(&next_position) {
                let mut _path = path.clone();
                _path.push(next_position);
                paths.push_back(_path);
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "
.....
.S-7.
.|.|.
.L-J.
.....
    ";
    const EXAMPLE2: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE1)), 4);
        assert_eq!(part1(parse_input(EXAMPLE2)), 8);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 6864);
        // 6834
    }

    const EXAMPLE3: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
    ";
    const EXAMPLE4: &str = "
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
    ";
    const EXAMPLE5: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
    ";
    const EXAMPLE6: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
    ";

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE1)), 4);
        assert_eq!(part2(parse_input(EXAMPLE2)), 8);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 6864);

        // assert_eq!(part2(parse_input(EXAMPLE3)), 4);
        // assert_eq!(part2(parse_input(EXAMPLE4)), 4);
        // assert_eq!(part2(parse_input(EXAMPLE5)), 8);
        // assert_eq!(part2(parse_input(EXAMPLE6)), 10);
        // assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    }
}
