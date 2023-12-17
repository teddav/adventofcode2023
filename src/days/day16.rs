use std::collections::{HashSet, VecDeque};

use aoc2023::get_input;

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

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(map: Vec<Vec<char>>) -> usize {
    let max = (map[0].len() - 1, map.len() - 1);

    let mut beams = VecDeque::new();
    let mut seen = HashSet::new();

    for first_direction in Direction::Right.next(map[0][0]) {
        beams.push_back((Position(0, 0), first_direction))
    }

    while let Some((current_pos, current_dir)) = beams.pop_front() {
        if seen.contains(&(current_pos, current_dir)) {
            continue;
        }
        // println!("-> {:?} {:?}", current_pos, current_dir);
        seen.insert((current_pos, current_dir));

        if let Some(next_pos) = current_pos.next(current_dir, max) {
            for next_dir in current_dir.next(map[next_pos.1][next_pos.0]) {
                beams.push_back((next_pos, next_dir));
            }
        }
    }

    let energized: HashSet<Position> = HashSet::from_iter(seen.iter().map(|v| v.0));
    energized.len()
}
// fn part2(input: Vec<&str>) {}

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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
