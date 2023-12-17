use aoc2023::get_input;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn next(&self, mirror: char) -> Vec<Self> {
        use Direction::*;
        match mirror {
            '.' => vec![*self],
            '|' => match self {
                Left | Right => {
                    vec![Up, Down]
                }
                _ => vec![*self],
            },
            '-' => match self {
                Up | Down => {
                    vec![Left, Right]
                }
                _ => vec![*self],
            },
            '/' => match self {
                Left => vec![Down],
                Right => vec![Up],
                Up => vec![Right],
                Down => vec![Left],
            },
            '\u{005C}' => match self {
                Left => vec![Up],
                Right => vec![Down],
                Up => vec![Left],
                Down => vec![Right],
            },
            _ => panic!("unknown mirror"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);
impl Position {
    fn next(&self, dir: Direction, max: (usize, usize)) -> Option<Position> {
        use Direction::*;
        match dir {
            Left => {
                if self.0 > 0 {
                    Some(Position(self.0 - 1, self.1))
                } else {
                    None
                }
            }
            Right => {
                if (self.0 as usize) < max.0 {
                    Some(Position(self.0 + 1, self.1))
                } else {
                    None
                }
            }
            Up => {
                if self.1 > 0 {
                    Some(Position(self.0, self.1 - 1))
                } else {
                    None
                }
            }
            Down => {
                if (self.1 as usize) < max.1 {
                    Some(Position(self.0, self.1 + 1))
                } else {
                    None
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(map: Vec<Vec<char>>) -> usize {
    let energized: HashSet<Position> = get_energized(&map, Position(0, 0), Direction::Right);
    energized.len()
}

fn get_energized(
    map: &Vec<Vec<char>>,
    start_pos: Position,
    start_dir: Direction,
) -> HashSet<Position> {
    let max = (map[0].len() - 1, map.len() - 1);

    let mut beams = VecDeque::new();
    let mut seen = HashSet::new();

    for first_direction in start_dir.next(map[start_pos.1][start_pos.0]) {
        beams.push_back((start_pos, first_direction))
    }

    while let Some((current_pos, current_dir)) = beams.pop_front() {
        if seen.contains(&(current_pos, current_dir)) {
            continue;
        }
        seen.insert((current_pos, current_dir));

        if let Some(next_pos) = current_pos.next(current_dir, max) {
            for next_dir in current_dir.next(map[next_pos.1][next_pos.0]) {
                beams.push_back((next_pos, next_dir));
            }
        }
    }
    HashSet::from_iter(seen.iter().map(|v| v.0))
}

fn part2(map: Vec<Vec<char>>) -> usize {
    let max = (map[0].len() - 1, map.len() - 1);
    let mut energized = vec![];

    for x in 0..=max.0 {
        let _energized = get_energized(&map, Position(x, 0), Direction::Down);
        energized.push(_energized.len());

        let _energized = get_energized(&map, Position(x, max.1), Direction::Up);
        energized.push(_energized.len());
    }

    for y in 0..=max.1 {
        let _energized = get_energized(&map, Position(0, y), Direction::Right);
        energized.push(_energized.len());

        let _energized = get_energized(&map, Position(max.0, y), Direction::Left);
        energized.push(_energized.len());
    }

    *energized.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 46);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 8323);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 51);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    }
}
