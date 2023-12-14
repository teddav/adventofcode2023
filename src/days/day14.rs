use aoc2023::get_input;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Pos(usize, usize);

struct HashedPositions(HashSet<Pos>);

impl HashedPositions {
    fn from_vec(item: &Vec<Pos>) -> Self {
        let set = HashSet::from_iter(item.iter().cloned());
        HashedPositions(set)
    }
}

impl PartialEq for HashedPositions {
    fn eq(&self, other: &HashedPositions) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}
impl Eq for HashedPositions {}
impl Hash for HashedPositions {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut a: Vec<&Pos> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));

    let start = Instant::now();
    println!("Part2: {}", part2(parse_input(&input), 1000000000));
    println!("Part2 time: {:?}", start.elapsed());
}

fn parse_input(input: &str) -> (Vec<Pos>, Vec<Pos>, usize, usize) {
    let mut rounds = vec![];
    let mut cubes = vec![];
    let mut height = 0;
    let mut width = 0;

    for (y, line) in input.trim().split("\n").enumerate() {
        for (x, c) in line.char_indices() {
            if c == 'O' {
                rounds.push(Pos(x, y));
            }
            if c == '#' {
                cubes.push(Pos(x, y));
            }
            width = x;
        }
        height = y + 1;
    }
    (rounds, cubes, height, width)
}

fn part1((mut rounds, cubes, height, width): (Vec<Pos>, Vec<Pos>, usize, usize)) -> usize {
    tilt(&Direction::North, &mut rounds, &cubes, (width, height - 1));
    rounds.iter().map(|rock| height - rock.1).sum()
}

fn part2(
    (mut rounds, cubes, height, width): (Vec<Pos>, Vec<Pos>, usize, usize),
    cycles: usize,
) -> usize {
    let directions = vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut memo: HashMap<HashedPositions, usize> = HashMap::new();
    let mut current_cycle = 0;

    while current_cycle < cycles {
        for dir in &directions {
            tilt(dir, &mut rounds, &cubes, (width, height - 1));
        }

        let hashed = HashedPositions::from_vec(&rounds);
        if let Some(&cycle) = memo.get(&hashed) {
            let repeat_loop = current_cycle - cycle;
            let advance_by = (cycles - 1 - current_cycle) / repeat_loop;
            current_cycle += advance_by * repeat_loop;
        }
        memo.insert(hashed, current_cycle);

        current_cycle += 1;
    }

    // print_map(&rounds, &cubes, height, width);
    rounds.iter().map(|rock| height - rock.1).sum()
}

#[allow(dead_code)]
fn print_map(rounds: &Vec<Pos>, cubes: &Vec<Pos>, height: usize, width: usize) {
    let mut map: Vec<Vec<char>> = (0..height).map(|_| vec!['.'].repeat(width + 1)).collect();
    for r in rounds {
        map[r.1][r.0] = 'O';
    }
    for r in cubes {
        map[r.1][r.0] = '#';
    }
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}

fn tilt(direction: &Direction, rounds: &mut Vec<Pos>, cubes: &Vec<Pos>, max: (usize, usize)) {
    for i in 0..rounds.len() {
        let pos = rounds[i];

        let mut to_move = match direction {
            Direction::East | Direction::West => pos.0,
            Direction::North | Direction::South => pos.1,
        };

        let condition = |v| match direction {
            Direction::North | Direction::West => v > 0,
            Direction::East => v < max.0,
            Direction::South => v < max.1,
        };
        while condition(to_move) {
            to_move = match direction {
                Direction::North | Direction::West => to_move - 1,
                Direction::East | Direction::South => to_move + 1,
            };

            let next_pos = match direction {
                Direction::East | Direction::West => Pos(to_move, pos.1),
                Direction::North | Direction::South => Pos(pos.0, to_move),
            };

            // if it's a cube, we cannot move, we stop
            if cubes.contains(&next_pos) {
                break;
            }

            // if it's a round, we need to check higher, to see if there is an empty space
            if rounds.contains(&next_pos) {
                continue;
            }

            match direction {
                Direction::East | Direction::West => rounds[i].0 = to_move,
                Direction::North | Direction::South => rounds[i].1 = to_move,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 136);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 111979);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE), 1), 87);
        assert_eq!(part2(parse_input(EXAMPLE), 2), 69);
        assert_eq!(part2(parse_input(EXAMPLE), 3), 69);
        assert_eq!(part2(parse_input(EXAMPLE), 20), 64);
        assert_eq!(part2(parse_input(EXAMPLE), 100), 68);
        assert_eq!(part2(parse_input(EXAMPLE), 1000), 64);
        assert_eq!(part2(parse_input(&get_input!(file!())), 1), 103356);
        // assert_eq!(part2(parse_input(&get_input!(file!())), 1000000000), 102055);
    }
}
